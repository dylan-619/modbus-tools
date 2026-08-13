use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use crate::models::{ReadRequest, ReadResult, TransportConfig};
use crate::error::ModbusToolError;
use crate::connection::commands::WorkerCommand;
use crate::connection::tcp_worker::TcpWorker;
use crate::connection::rtu_worker::RtuWorker;
use tokio_util::sync::CancellationToken;

#[derive(Clone)]
pub struct ConnectionManager {
    connections: Arc<Mutex<HashMap<String, mpsc::Sender<WorkerCommand>>>>,
    cancel_token: CancellationToken,
}

impl ConnectionManager {
    pub fn new(cancel_token: CancellationToken) -> Self {
        Self {
            connections: Arc::new(Mutex::new(HashMap::new())),
            cancel_token,
        }
    }

    pub async fn connect(
        &self,
        connection_id: String,
        config: TransportConfig,
    ) -> Result<(), ModbusToolError> {
        let (tx, rx) = mpsc::channel(32);
        let token = self.cancel_token.child_token();

        match config {
            TransportConfig::Tcp(cfg) => {
                let worker = TcpWorker::new(cfg, rx, token);
                tokio::spawn(worker.run());
            }
            TransportConfig::Rtu(cfg) => {
                let worker = RtuWorker::new(cfg, rx, token);
                tokio::spawn(worker.run());
            }
        }

        let mut conns = self.connections.lock().await;
        // If there's an existing connection, shut it down
        if let Some(old_tx) = conns.insert(connection_id, tx) {
            let _ = old_tx.send(WorkerCommand::Shutdown).await;
        }

        Ok(())
    }

    pub async fn disconnect(&self, connection_id: &str) -> Result<(), ModbusToolError> {
        let mut conns = self.connections.lock().await;
        if let Some(tx) = conns.remove(connection_id) {
            let _ = tx.send(WorkerCommand::Shutdown).await;
            Ok(())
        } else {
            Err(ModbusToolError::ConnectionNotFound)
        }
    }

    pub async fn send_read_request(&self, request: ReadRequest) -> Result<ReadResult, ModbusToolError> {
        let tx = {
            let conns = self.connections.lock().await;
            conns.get(&request.connection_id).cloned()
        };

        if let Some(tx) = tx {
            let (resp_tx, resp_rx) = tokio::sync::oneshot::channel();
            if tx.send(WorkerCommand::Read { request, response_tx: resp_tx }).await.is_err() {
                return Err(ModbusToolError::ConnectionNotFound); // Worker died
            }
            resp_rx.await.map_err(|_| ModbusToolError::ConnectionNotFound)?
        } else {
            Err(ModbusToolError::ConnectionNotFound)
        }
    }

    pub async fn send_write_request(&self, request: crate::models::WriteRequest) -> Result<crate::models::WriteResult, ModbusToolError> {
        let tx = {
            let conns = self.connections.lock().await;
            conns.get(&request.connection_id).cloned()
        };

        if let Some(tx) = tx {
            let (resp_tx, resp_rx) = tokio::sync::oneshot::channel();
            if tx.send(WorkerCommand::Write { request, response_tx: resp_tx }).await.is_err() {
                return Err(ModbusToolError::ConnectionNotFound); // Worker died
            }
            resp_rx.await.map_err(|_| ModbusToolError::ConnectionNotFound)?
        } else {
            Err(ModbusToolError::ConnectionNotFound)
        }
    }
}
