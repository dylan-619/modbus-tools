use crate::models::{ParsedValue, ValueTransform};

pub fn apply_transform(value: &ParsedValue, transform: &ValueTransform) -> (Option<f64>, String) {
    let numeric_val = match value {
        ParsedValue::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
        ParsedValue::UInt16(v) => Some(*v as f64),
        ParsedValue::Int16(v) => Some(*v as f64),
        ParsedValue::UInt32(v) => Some(*v as f64),
        ParsedValue::Int32(v) => Some(*v as f64),
        ParsedValue::Float32(v) => Some(*v as f64),
        ParsedValue::UInt64(v) => Some(*v as f64),
        ParsedValue::Int64(v) => Some(*v as f64),
        ParsedValue::Float64(v) => Some(*v),
        ParsedValue::Ascii(_) | ParsedValue::Raw(_) => None,
    };

    if let Some(val) = numeric_val {
        let transformed = val * transform.scale + transform.offset;
        let mut display = if let Some(decimals) = transform.decimals {
            format!("{:.*}", decimals as usize, transformed)
        } else {
            format!("{:g}", transformed)
        };

        if let Some(unit) = &transform.unit {
            if !unit.is_empty() {
                display.push_str(" ");
                display.push_str(unit);
            }
        }
        (Some(transformed), display)
    } else {
        match value {
            ParsedValue::Ascii(s) => (None, s.clone()),
            ParsedValue::Raw(bytes) => (None, format!("{:02X?}", bytes)),
            _ => unreachable!(),
        }
    }
}
