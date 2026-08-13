use crate::error::ModbusToolError;
use crate::protocol::exception::ModbusExceptionCode;

#[derive(Debug, Clone, PartialEq)]
pub enum ModbusRequest {
    ReadCoils { address: u16, quantity: u16 },
    ReadDiscreteInputs { address: u16, quantity: u16 },
    ReadHoldingRegisters { address: u16, quantity: u16 },
    ReadInputRegisters { address: u16, quantity: u16 },
    WriteSingleCoil { address: u16, value: bool },
    WriteSingleRegister { address: u16, value: u16 },
    WriteMultipleCoils { address: u16, quantity: u16, values: Vec<u8> },
    WriteMultipleRegisters { address: u16, quantity: u16, values: Vec<u16> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModbusResponse {
    ReadBits(Vec<bool>),
    ReadRegisters(Vec<u16>),
    WriteSingleCoil { address: u16, value: bool },
    WriteSingleRegister { address: u16, value: u16 },
    WriteMultiple { address: u16, quantity: u16 },
    Exception(u8, ModbusExceptionCode),
}

impl ModbusRequest {
    pub fn function_code(&self) -> u8 {
        match self {
            ModbusRequest::ReadCoils { .. } => 0x01,
            ModbusRequest::ReadDiscreteInputs { .. } => 0x02,
            ModbusRequest::ReadHoldingRegisters { .. } => 0x03,
            ModbusRequest::ReadInputRegisters { .. } => 0x04,
            ModbusRequest::WriteSingleCoil { .. } => 0x05,
            ModbusRequest::WriteSingleRegister { .. } => 0x06,
            ModbusRequest::WriteMultipleCoils { .. } => 0x0F,
            ModbusRequest::WriteMultipleRegisters { .. } => 0x10,
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(self.function_code());
        match self {
            ModbusRequest::ReadCoils { address, quantity }
            | ModbusRequest::ReadDiscreteInputs { address, quantity }
            | ModbusRequest::ReadHoldingRegisters { address, quantity }
            | ModbusRequest::ReadInputRegisters { address, quantity } => {
                buf.extend_from_slice(&address.to_be_bytes());
                buf.extend_from_slice(&quantity.to_be_bytes());
            }
            ModbusRequest::WriteSingleCoil { address, value } => {
                buf.extend_from_slice(&address.to_be_bytes());
                buf.extend_from_slice(if *value { &[0xFF, 0x00] } else { &[0x00, 0x00] });
            }
            ModbusRequest::WriteSingleRegister { address, value } => {
                buf.extend_from_slice(&address.to_be_bytes());
                buf.extend_from_slice(&value.to_be_bytes());
            }
            ModbusRequest::WriteMultipleCoils { address, quantity, values } => {
                buf.extend_from_slice(&address.to_be_bytes());
                buf.extend_from_slice(&quantity.to_be_bytes());
                buf.push(values.len() as u8);
                buf.extend_from_slice(values);
            }
            ModbusRequest::WriteMultipleRegisters { address, quantity, values } => {
                buf.extend_from_slice(&address.to_be_bytes());
                buf.extend_from_slice(&quantity.to_be_bytes());
                buf.push((values.len() * 2) as u8);
                for v in values {
                    buf.extend_from_slice(&v.to_be_bytes());
                }
            }
        }
        buf
    }
}

pub fn decode_response(
    function_code: u8,
    data: &[u8],
) -> Result<ModbusResponse, ModbusToolError> {
    if data.is_empty() {
        return Err(ModbusToolError::IncompleteFrame);
    }

    // Check for exception response (function_code | 0x80)
    let is_exception = data[0] == (function_code | 0x80);
    if is_exception {
        if data.len() < 2 {
            return Err(ModbusToolError::IncompleteFrame);
        }
        let exception_code = data[1];
        return Ok(ModbusResponse::Exception(
            function_code,
            ModbusExceptionCode::from(exception_code),
        ));
    }

    // Normal response must match the requested function code
    if data[0] != function_code {
        return Err(ModbusToolError::FunctionCodeMismatch);
    }

    match function_code {
        0x01 | 0x02 => {
            if data.len() < 2 { return Err(ModbusToolError::IncompleteFrame); }
            let byte_count = data[1] as usize;
            if data.len() < 2 + byte_count { return Err(ModbusToolError::IncompleteFrame); }
            let payload = &data[2..2 + byte_count];
            
            let mut bits = Vec::new();
            for byte in payload {
                for i in 0..8 {
                    bits.push((byte & (1 << i)) != 0);
                }
            }
            Ok(ModbusResponse::ReadBits(bits))
        }
        0x03 | 0x04 => {
            if data.len() < 2 { return Err(ModbusToolError::IncompleteFrame); }
            let byte_count = data[1] as usize;
            if data.len() < 2 + byte_count { return Err(ModbusToolError::IncompleteFrame); }
            let payload = &data[2..2 + byte_count];
            
            if byte_count % 2 != 0 {
                return Err(ModbusToolError::InvalidByteCount);
            }
            let mut registers = Vec::with_capacity(byte_count / 2);
            for chunk in payload.chunks_exact(2) {
                registers.push(u16::from_be_bytes([chunk[0], chunk[1]]));
            }
            Ok(ModbusResponse::ReadRegisters(registers))
        }
        0x05 => {
            if data.len() < 5 { return Err(ModbusToolError::IncompleteFrame); }
            let address = u16::from_be_bytes([data[1], data[2]]);
            let value = u16::from_be_bytes([data[3], data[4]]);
            Ok(ModbusResponse::WriteSingleCoil { address, value: value == 0xFF00 })
        }
        0x06 => {
            if data.len() < 5 { return Err(ModbusToolError::IncompleteFrame); }
            let address = u16::from_be_bytes([data[1], data[2]]);
            let value = u16::from_be_bytes([data[3], data[4]]);
            Ok(ModbusResponse::WriteSingleRegister { address, value })
        }
        0x0F | 0x10 => {
            if data.len() < 5 { return Err(ModbusToolError::IncompleteFrame); }
            let address = u16::from_be_bytes([data[1], data[2]]);
            let quantity = u16::from_be_bytes([data[3], data[4]]);
            Ok(ModbusResponse::WriteMultiple { address, quantity })
        }
        _ => Err(ModbusToolError::UnsupportedFunctionCode(function_code)),
    }
}
