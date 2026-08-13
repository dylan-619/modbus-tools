use crate::error::ModbusToolError;
use crate::models::{ByteLayout, DataType, ParsedValue};
use crate::parser::numeric::parse_numeric_value;
use crate::parser::byte_order::reorder_bytes_16;

pub fn parse_data(
    registers: &[u16],
    data_type: DataType,
    layout: ByteLayout,
) -> Result<ParsedValue, ModbusToolError> {
    match data_type {
        DataType::Bool => {
            // Usually Bool is parsed directly from coils, but if read from registers, LSB is taken
            if registers.is_empty() {
                return Err(ModbusToolError::InvalidRegisterCount);
            }
            Ok(ParsedValue::Bool((registers[0] & 1) != 0))
        }
        DataType::Ascii { registers: reg_count } => {
            if registers.len() != reg_count as usize {
                return Err(ModbusToolError::InvalidRegisterCount);
            }
            let mut bytes = Vec::with_capacity(registers.len() * 2);
            for reg in registers {
                let r_bytes = reg.to_be_bytes();
                let reordered = reorder_bytes_16(r_bytes, layout);
                bytes.extend_from_slice(&reordered);
            }
            // Strip null bytes
            bytes.retain(|&b| b != 0);
            let s = String::from_utf8(bytes).unwrap_or_else(|_| "Invalid ASCII".into());
            Ok(ParsedValue::Ascii(s))
        }
        DataType::Raw { registers: reg_count } => {
            if registers.len() != reg_count as usize {
                return Err(ModbusToolError::InvalidRegisterCount);
            }
            let mut bytes = Vec::with_capacity(registers.len() * 2);
            for reg in registers {
                let r_bytes = reg.to_be_bytes();
                let reordered = reorder_bytes_16(r_bytes, layout);
                bytes.extend_from_slice(&reordered);
            }
            Ok(ParsedValue::Raw(bytes))
        }
        _ => parse_numeric_value(registers, data_type, layout),
    }
}
