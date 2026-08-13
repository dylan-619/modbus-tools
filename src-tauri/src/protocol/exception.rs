#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModbusExceptionCode {
    IllegalFunction,
    IllegalDataAddress,
    IllegalDataValue,
    SlaveDeviceFailure,
    Acknowledge,
    SlaveDeviceBusy,
    MemoryParityError,
    GatewayPathUnavailable,
    GatewayTargetFailedToRespond,
    Unknown(u8),
}

impl From<u8> for ModbusExceptionCode {
    fn from(code: u8) -> Self {
        match code {
            0x01 => ModbusExceptionCode::IllegalFunction,
            0x02 => ModbusExceptionCode::IllegalDataAddress,
            0x03 => ModbusExceptionCode::IllegalDataValue,
            0x04 => ModbusExceptionCode::SlaveDeviceFailure,
            0x05 => ModbusExceptionCode::Acknowledge,
            0x06 => ModbusExceptionCode::SlaveDeviceBusy,
            0x08 => ModbusExceptionCode::MemoryParityError,
            0x0A => ModbusExceptionCode::GatewayPathUnavailable,
            0x0B => ModbusExceptionCode::GatewayTargetFailedToRespond,
            _ => ModbusExceptionCode::Unknown(code),
        }
    }
}

impl ModbusExceptionCode {
    pub fn to_u8(&self) -> u8 {
        match self {
            ModbusExceptionCode::IllegalFunction => 0x01,
            ModbusExceptionCode::IllegalDataAddress => 0x02,
            ModbusExceptionCode::IllegalDataValue => 0x03,
            ModbusExceptionCode::SlaveDeviceFailure => 0x04,
            ModbusExceptionCode::Acknowledge => 0x05,
            ModbusExceptionCode::SlaveDeviceBusy => 0x06,
            ModbusExceptionCode::MemoryParityError => 0x08,
            ModbusExceptionCode::GatewayPathUnavailable => 0x0A,
            ModbusExceptionCode::GatewayTargetFailedToRespond => 0x0B,
            ModbusExceptionCode::Unknown(code) => *code,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ModbusExceptionCode::IllegalFunction => "Illegal Function: The function code received in the query is not an allowable action for the server.",
            ModbusExceptionCode::IllegalDataAddress => "Illegal Data Address: The data address received in the query is not an allowable address for the server.",
            ModbusExceptionCode::IllegalDataValue => "Illegal Data Value: A value contained in the query data field is not an allowable value for server.",
            ModbusExceptionCode::SlaveDeviceFailure => "Slave Device Failure: An unrecoverable error occurred while the server was attempting to perform the requested action.",
            ModbusExceptionCode::Acknowledge => "Acknowledge: The server has accepted the request and is processing it, but a long duration of time will be required to do so.",
            ModbusExceptionCode::SlaveDeviceBusy => "Slave Device Busy: The server is engaged in processing a long-duration program command.",
            ModbusExceptionCode::MemoryParityError => "Memory Parity Error: The server attempted to read record file, but detected a parity error in the memory.",
            ModbusExceptionCode::GatewayPathUnavailable => "Gateway Path Unavailable: Gateway was unable to allocate an internal communication path from the input port to the output port.",
            ModbusExceptionCode::GatewayTargetFailedToRespond => "Gateway Target Failed To Respond: No response was obtained from the target device.",
            ModbusExceptionCode::Unknown(_) => "Unknown exception code.",
        }
    }
}
