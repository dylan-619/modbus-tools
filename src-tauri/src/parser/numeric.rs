use crate::error::ModbusToolError;
use crate::models::{ByteLayout, DataType, ParsedValue};
use crate::parser::byte_order::{reorder_bytes_16, reorder_bytes_32, reorder_bytes_64};

pub fn parse_numeric_value(
    registers: &[u16],
    data_type: DataType,
    layout: ByteLayout,
) -> Result<ParsedValue, ModbusToolError> {
    match data_type {
        DataType::UInt16 | DataType::Int16 => {
            if registers.len() != 1 {
                return Err(ModbusToolError::InvalidRegisterCount);
            }
            let bytes = registers[0].to_be_bytes();
            let reordered = reorder_bytes_16(bytes, layout);
            
            match data_type {
                DataType::UInt16 => Ok(ParsedValue::UInt16(u16::from_be_bytes(reordered))),
                DataType::Int16 => Ok(ParsedValue::Int16(i16::from_be_bytes(reordered))),
                _ => unreachable!(),
            }
        }
        DataType::UInt32 | DataType::Int32 | DataType::Float32 => {
            if registers.len() != 2 {
                return Err(ModbusToolError::InvalidRegisterCount);
            }
            let r0 = registers[0].to_be_bytes();
            let r1 = registers[1].to_be_bytes();
            let bytes = [r0[0], r0[1], r1[0], r1[1]];
            let reordered = reorder_bytes_32(bytes, layout);
            
            match data_type {
                DataType::UInt32 => Ok(ParsedValue::UInt32(u32::from_be_bytes(reordered))),
                DataType::Int32 => Ok(ParsedValue::Int32(i32::from_be_bytes(reordered))),
                DataType::Float32 => Ok(ParsedValue::Float32(f32::from_be_bytes(reordered))),
                _ => unreachable!(),
            }
        }
        DataType::UInt64 | DataType::Int64 | DataType::Float64 => {
            if registers.len() != 4 {
                return Err(ModbusToolError::InvalidRegisterCount);
            }
            let r0 = registers[0].to_be_bytes();
            let r1 = registers[1].to_be_bytes();
            let r2 = registers[2].to_be_bytes();
            let r3 = registers[3].to_be_bytes();
            let bytes = [r0[0], r0[1], r1[0], r1[1], r2[0], r2[1], r3[0], r3[1]];
            let reordered = reorder_bytes_64(bytes, layout);
            
            match data_type {
                DataType::UInt64 => Ok(ParsedValue::UInt64(u64::from_be_bytes(reordered))),
                DataType::Int64 => Ok(ParsedValue::Int64(i64::from_be_bytes(reordered))),
                DataType::Float64 => Ok(ParsedValue::Float64(f64::from_be_bytes(reordered))),
                _ => unreachable!(),
            }
        }
        _ => Err(ModbusToolError::ParseError("Expected numeric type".into())),
    }
}
