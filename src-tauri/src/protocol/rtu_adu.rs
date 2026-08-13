use crate::error::ModbusToolError;
use crate::protocol::crc16::modbus_crc16;

pub fn encode_rtu_request(slave_id: u8, pdu: &[u8]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(1 + pdu.len() + 2);
    buf.push(slave_id);
    buf.extend_from_slice(pdu);
    
    let crc = modbus_crc16(&buf);
    buf.extend_from_slice(&crc.to_le_bytes()); // Modbus RTU sends CRC LO first, then HI
    
    buf
}

pub fn decode_rtu_frame(data: &[u8]) -> Result<(u8, &[u8]), ModbusToolError> {
    if data.len() < 4 {
        return Err(ModbusToolError::IncompleteFrame);
    }
    
    let slave_id = data[0];
    
    // Check CRC
    let crc_received = u16::from_le_bytes([data[data.len() - 2], data[data.len() - 1]]);
    let crc_calculated = modbus_crc16(&data[..data.len() - 2]);
    
    if crc_received != crc_calculated {
        return Err(ModbusToolError::CrcMismatch);
    }
    
    let pdu = &data[1..data.len() - 2];
    Ok((slave_id, pdu))
}
