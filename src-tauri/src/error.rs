use serde::{Serialize, Serializer};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModbusToolError {
    // Connection Errors
    #[error("TCP Connection Timeout")]
    TcpConnectTimeout,
    #[error("TCP Disconnected")]
    TcpDisconnected,
    #[error("Serial Port Not Found")]
    SerialPortNotFound,
    #[error("Serial Port Busy")]
    SerialPortBusy,
    #[error("Failed to open Serial Port: {0}")]
    SerialOpenFailed(String),

    // Protocol Errors
    #[error("Response Timeout")]
    ResponseTimeout,
    #[error("Incomplete Frame")]
    IncompleteFrame,
    #[error("CRC Mismatch")]
    CrcMismatch,
    #[error("Transaction ID Mismatch")]
    TransactionIdMismatch,
    #[error("Unit ID Mismatch")]
    UnitIdMismatch,
    #[error("Function Code Mismatch")]
    FunctionCodeMismatch,
    #[error("Invalid Byte Count")]
    InvalidByteCount,
    #[error("Modbus Exception: {0}")]
    ModbusException(u8),
    #[error("Unsupported Function Code: {0}")]
    UnsupportedFunctionCode(u8),

    // Parameter Errors
    #[error("Address Out of Range")]
    AddressOutOfRange,
    #[error("Invalid Register Count")]
    InvalidRegisterCount,
    #[error("Parse Error: {0}")]
    ParseError(String),

    // State Errors
    #[error("Connection Not Found")]
    ConnectionNotFound,
    #[error("Polling Already Active")]
    PollingAlreadyActive,
    #[error("Polling Not Found")]
    PollingNotFound,

    // Operation Errors
    #[error("Operation Cancelled")]
    Cancelled,
    #[error("Config Save Error: {0}")]
    ConfigSaveError(String),
}

impl Serialize for ModbusToolError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
