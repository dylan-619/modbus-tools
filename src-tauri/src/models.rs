use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TransportConfig {
    #[serde(rename = "tcp")]
    Tcp(TcpConfig),
    #[serde(rename = "rtu")]
    Rtu(RtuConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TcpConfig {
    pub host: String,
    pub port: u16,
    pub unit_id: u8,
    pub connect_timeout_ms: u64,
    pub response_timeout_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RtuConfig {
    pub port_name: String,
    pub slave_id: u8,
    pub baud_rate: u32,
    pub data_bits: u8,
    pub stop_bits: u8,
    pub parity: String,
    pub response_timeout_ms: u64,
    pub inter_request_delay_ms: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataType {
    Bool,
    UInt16,
    Int16,
    UInt32,
    Int32,
    Float32,
    UInt64,
    Int64,
    Float64,
    Ascii { registers: u16 },
    Raw { registers: u16 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ByteLayout {
    Ab,
    Ba,
    Abcd,
    Cdab,
    Badc,
    Dcba,
    Abcdefgh,
    Ghefcdab,
    Badcfehg,
    Hgfedcba,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueTransform {
    pub scale: f64,
    pub offset: f64,
    pub decimals: Option<u8>,
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReadFunction {
    ReadCoils = 1,
    ReadDiscreteInputs = 2,
    ReadHoldingRegisters = 3,
    ReadInputRegisters = 4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadRequest {
    pub connection_id: String,
    pub function: ReadFunction,
    pub address: u16,
    pub data_type: DataType,
    pub layout: ByteLayout,
    pub transform: ValueTransform,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParsedValue {
    Bool(bool),
    UInt16(u16),
    Int16(i16),
    UInt32(u32),
    Int32(i32),
    Float32(f32),
    UInt64(u64),
    Int64(i64),
    Float64(f64),
    Ascii(String),
    Raw(Vec<u8>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadResult {
    pub timestamp_ms: i64,
    pub raw_value: ParsedValue,
    pub transformed_value: Option<f64>,
    pub display_value: String,
    pub registers: Vec<u16>,
    pub data_bytes: Vec<u8>,
    pub request_frame: Vec<u8>,
    pub response_frame: Vec<u8>,
    pub elapsed_ms: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WriteFunction {
    WriteSingleCoil = 5,
    WriteSingleRegister = 6,
    WriteMultipleCoils = 15,
    WriteMultipleRegisters = 16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteRequest {
    pub connection_id: String,
    pub function: WriteFunction,
    pub address: u16,
    pub values: Vec<u16>, // Represent coils as 0xFF00 / 0x0000 or 1/0 for single coil, packed bits for multiple coils, or u16 for registers
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteResult {
    pub timestamp_ms: i64,
    pub request_frame: Vec<u8>,
    pub response_frame: Vec<u8>,
    pub elapsed_ms: u64,
}
