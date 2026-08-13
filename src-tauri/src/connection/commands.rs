use crate::models::{ReadRequest, ReadResult};
use crate::error::ModbusToolError;
use tokio::sync::oneshot;

pub enum WorkerCommand {
    Read {
        request: ReadRequest,
        response_tx: oneshot::Sender<Result<ReadResult, ModbusToolError>>,
    },
    Write {
        request: crate::models::WriteRequest,
        response_tx: oneshot::Sender<Result<crate::models::WriteResult, ModbusToolError>>,
    },
    Shutdown,
}
