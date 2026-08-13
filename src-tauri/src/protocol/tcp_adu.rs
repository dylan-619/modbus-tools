use bytes::{Buf, BytesMut};
use crate::error::ModbusToolError;

pub struct TcpHeader {
    pub transaction_id: u16,
    pub protocol_id: u16,
    pub length: u16,
    pub unit_id: u8,
}

pub fn encode_tcp_request(
    transaction_id: u16,
    unit_id: u8,
    pdu: &[u8],
) -> Vec<u8> {
    let length = (1 + pdu.len()) as u16; // 1 byte for unit_id + pdu length
    let mut buf = Vec::with_capacity(7 + pdu.len());
    
    buf.extend_from_slice(&transaction_id.to_be_bytes());
    buf.extend_from_slice(&0u16.to_be_bytes()); // Protocol ID = 0
    buf.extend_from_slice(&length.to_be_bytes());
    buf.push(unit_id);
    buf.extend_from_slice(pdu);
    
    buf
}

pub fn decode_tcp_response_header(src: &mut BytesMut) -> Result<Option<(TcpHeader, usize)>, ModbusToolError> {
    if src.len() < 7 {
        return Ok(None); // Not enough data for header
    }

    let mut temp = &src[..7];
    let transaction_id = temp.get_u16();
    let protocol_id = temp.get_u16();
    let length = temp.get_u16();
    let unit_id = temp.get_u8();

    if protocol_id != 0 {
        return Err(ModbusToolError::ParseError("Invalid Protocol ID".into()));
    }

    let pdu_length = (length - 1) as usize;
    if src.len() < 7 + pdu_length {
        return Ok(None); // Not enough data for full frame
    }

    Ok(Some((
        TcpHeader {
            transaction_id,
            protocol_id,
            length,
            unit_id,
        },
        7 + pdu_length, // total frame size
    )))
}
