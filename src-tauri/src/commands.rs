use crate::state::AppState;
use crate::models::{TransportConfig, ReadRequest, ReadResult};
use crate::error::ModbusToolError;
use serde::Serialize;
use tauri::State;

#[tauri::command]
pub async fn connect_modbus(
    connection_id: String,
    config: TransportConfig,
    state: State<'_, AppState>,
) -> Result<(), ModbusToolError> {
    state.connection_manager.connect(connection_id, config).await
}

#[tauri::command]
pub async fn disconnect_modbus(
    connection_id: String,
    state: State<'_, AppState>,
) -> Result<(), ModbusToolError> {
    state.connection_manager.disconnect(&connection_id).await
}

#[tauri::command]
pub async fn read_value(
    request: ReadRequest,
    state: State<'_, AppState>,
) -> Result<ReadResult, ModbusToolError> {
    state.connection_manager.send_read_request(request).await
}

#[tauri::command]
pub async fn write_value(
    request: crate::models::WriteRequest,
    state: State<'_, AppState>,
) -> Result<crate::models::WriteResult, ModbusToolError> {
    state.connection_manager.send_write_request(request).await
}

#[derive(Serialize)]
pub struct SerialPortInfo {
    pub name: String,
    pub port_type: String, // Simplified
}

#[tauri::command]
pub async fn list_serial_ports() -> Result<Vec<SerialPortInfo>, ModbusToolError> {
    let ports = serialport::available_ports().map_err(|e| ModbusToolError::ParseError(e.to_string()))?;
    Ok(ports.into_iter().map(|p| SerialPortInfo {
        name: p.port_name,
        port_type: format!("{:?}", p.port_type),
    }).collect())
}
