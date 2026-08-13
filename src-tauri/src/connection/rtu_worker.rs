use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use tokio::time::timeout;
use tokio_util::sync::CancellationToken;
use tokio_serial::{SerialPortBuilderExt, SerialStream};
use crate::models::{RtuConfig, ReadResult, ParsedValue};
use crate::error::ModbusToolError;
use crate::connection::commands::WorkerCommand;
use crate::protocol::rtu_adu::{encode_rtu_request, decode_rtu_frame};
use crate::protocol::pdu::{decode_response, ModbusResponse};
use crate::parser::data_type::parse_data;
use crate::parser::transform::apply_transform;

pub struct RtuWorker {
    config: RtuConfig,
    rx: mpsc::Receiver<WorkerCommand>,
    cancel_token: CancellationToken,
}

impl RtuWorker {
    pub fn new(
        config: RtuConfig,
        rx: mpsc::Receiver<WorkerCommand>,
        cancel_token: CancellationToken,
    ) -> Self {
        Self {
            config,
            rx,
            cancel_token,
        }
    }

    pub async fn run(mut self) {
        let mut port = match self.open_port() {
            Ok(p) => p,
            Err(_) => return,
        };

        // Calculate T3.5 inter-frame delay
        let t3_5_us = if self.config.baud_rate > 19200 {
            1750
        } else {
            // 11 bits per char * 3.5 chars * 1,000,000 / baud
            (11 * 35 * 100000) / self.config.baud_rate
        };
        let t3_5_delay = Duration::from_micros(t3_5_us as u64);
        let inter_request_delay = Duration::from_millis(self.config.inter_request_delay_ms).max(t3_5_delay);

        loop {
            tokio::select! {
                _ = self.cancel_token.cancelled() => {
                    break;
                }
                cmd = self.rx.recv() => {
                    match cmd {
                        Some(WorkerCommand::Read { request, response_tx }) => {
                            let pdu = crate::protocol::pdu::ModbusRequest::ReadHoldingRegisters { // Simplified for now
                                address: request.address,
                                quantity: 1, // Calculate properly later
                            }.encode();
                            
                            let frame = encode_rtu_request(self.config.slave_id, &pdu);
                            let start_time = chrono::Utc::now();
                            
                            // Ensure inter-request delay
                            tokio::time::sleep(inter_request_delay).await;

                            // Clear rx buffer
                            let mut dummy = [0u8; 1024];
                            while let Ok(_) = timeout(Duration::from_millis(1), port.read(&mut dummy)).await {
                                // flush
                            }

                            if port.write_all(&frame).await.is_err() {
                                let _ = response_tx.send(Err(ModbusToolError::SerialPortNotFound)); // Actually could be disconnected
                                break;
                            }

                            // Read response
                            let mut response_buf = Vec::new();
                            let mut buf = [0u8; 256];
                            let read_timeout = Duration::from_millis(self.config.response_timeout_ms);
                            
                            let read_result = timeout(read_timeout, async {
                                loop {
                                    // Need to read until T3.5 silence
                                    match timeout(t3_5_delay.max(Duration::from_millis(5)), port.read(&mut buf)).await {
                                        Ok(Ok(0)) => {
                                            if !response_buf.is_empty() {
                                                break; // EOF
                                            }
                                        }
                                        Ok(Ok(n)) => {
                                            response_buf.extend_from_slice(&buf[..n]);
                                        }
                                        Ok(Err(_)) => {
                                            break; // Read error
                                        }
                                        Err(_) => {
                                            // T3.5 silence timeout reached! Frame is complete.
                                            if !response_buf.is_empty() {
                                                break;
                                            }
                                        }
                                    }
                                }
                                Ok::<Vec<u8>, ModbusToolError>(response_buf.clone())
                            }).await;

                            let elapsed = chrono::Utc::now().signed_duration_since(start_time).num_milliseconds() as u64;

                            match read_result {
                                Ok(Ok(full_frame)) => {
                                    if full_frame.is_empty() {
                                        let _ = response_tx.send(Err(ModbusToolError::ResponseTimeout));
                                        continue;
                                    }
                                    
                                    match decode_rtu_frame(&full_frame) {
                                        Ok((slave_id, pdu_data)) => {
                                            if slave_id != self.config.slave_id {
                                                // Ignore frame from another slave
                                                let _ = response_tx.send(Err(ModbusToolError::ParseError("Slave ID mismatch".into())));
                                                continue;
                                            }
                                            
                                            match decode_response(request.function as u8, pdu_data) {
                                                Ok(ModbusResponse::ReadRegisters(regs)) => {
                                                    match parse_data(&regs, request.data_type, request.layout) {
                                                        Ok(parsed) => {
                                                            let (transformed, display) = apply_transform(&parsed, &request.transform);
                                                            let _ = response_tx.send(Ok(ReadResult {
                                                                timestamp_ms: chrono::Utc::now().timestamp_millis(),
                                                                raw_value: parsed,
                                                                transformed_value: transformed,
                                                                display_value: display,
                                                                registers: regs,
                                                                data_bytes: pdu_data.to_vec(),
                                                                request_frame: frame,
                                                                response_frame: full_frame,
                                                                elapsed_ms: elapsed,
                                                            }));
                                                        }
                                                        Err(e) => { let _ = response_tx.send(Err(e)); }
                                                    }
                                                }
                                                Ok(ModbusResponse::Exception(_, code)) => {
                                                    let _ = response_tx.send(Err(ModbusToolError::ModbusException(code.to_u8())));
                                                }
                                                _ => {
                                                    let _ = response_tx.send(Err(ModbusToolError::ParseError("Invalid response type".into())));
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            let _ = response_tx.send(Err(e));
                                        }
                                    }
                                }
                                Ok(Err(_)) => { let _ = response_tx.send(Err(ModbusToolError::SerialPortNotFound)); break; }
                                Err(_) => { let _ = response_tx.send(Err(ModbusToolError::ResponseTimeout)); }
                            }
                        }
                        Some(WorkerCommand::Write { request, response_tx }) => {
                            let pdu = match request.function {
                                crate::models::WriteFunction::WriteSingleCoil => crate::protocol::pdu::ModbusRequest::WriteSingleCoil { address: request.address, value: request.values[0] != 0 }.encode(),
                                crate::models::WriteFunction::WriteSingleRegister => crate::protocol::pdu::ModbusRequest::WriteSingleRegister { address: request.address, value: request.values[0] }.encode(),
                                crate::models::WriteFunction::WriteMultipleCoils => {
                                    let mut packed = vec![0u8; (request.values.len() + 7) / 8];
                                    for (i, v) in request.values.iter().enumerate() {
                                        if *v != 0 { packed[i / 8] |= 1 << (i % 8); }
                                    }
                                    crate::protocol::pdu::ModbusRequest::WriteMultipleCoils { address: request.address, quantity: request.values.len() as u16, values: packed }.encode()
                                },
                                crate::models::WriteFunction::WriteMultipleRegisters => crate::protocol::pdu::ModbusRequest::WriteMultipleRegisters { address: request.address, quantity: request.values.len() as u16, values: request.values.clone() }.encode(),
                            };
                            
                            let frame = encode_rtu_request(self.config.slave_id, &pdu);
                            let start_time = chrono::Utc::now();
                            
                            tokio::time::sleep(inter_request_delay).await;

                            let mut dummy = [0u8; 1024];
                            while let Ok(_) = timeout(Duration::from_millis(1), port.read(&mut dummy)).await {}

                            if port.write_all(&frame).await.is_err() {
                                let _ = response_tx.send(Err(ModbusToolError::SerialPortNotFound));
                                break;
                            }

                            let mut response_buf = Vec::new();
                            let mut buf = [0u8; 256];
                            let read_timeout = Duration::from_millis(self.config.response_timeout_ms);
                            
                            let read_result = timeout(read_timeout, async {
                                loop {
                                    match timeout(t3_5_delay.max(Duration::from_millis(5)), port.read(&mut buf)).await {
                                        Ok(Ok(0)) => { if !response_buf.is_empty() { break; } }
                                        Ok(Ok(n)) => { response_buf.extend_from_slice(&buf[..n]); }
                                        Ok(Err(_)) => { break; }
                                        Err(_) => { if !response_buf.is_empty() { break; } }
                                    }
                                }
                                Ok::<Vec<u8>, ModbusToolError>(response_buf.clone())
                            }).await;

                            let elapsed = chrono::Utc::now().signed_duration_since(start_time).num_milliseconds() as u64;

                            match read_result {
                                Ok(Ok(full_frame)) => {
                                    if full_frame.is_empty() {
                                        let _ = response_tx.send(Err(ModbusToolError::ResponseTimeout));
                                        continue;
                                    }
                                    match decode_rtu_frame(&full_frame) {
                                        Ok((slave_id, pdu_data)) => {
                                            if slave_id != self.config.slave_id {
                                                let _ = response_tx.send(Err(ModbusToolError::ParseError("Slave ID mismatch".into())));
                                                continue;
                                            }
                                            match decode_response(pdu[0], pdu_data) {
                                                Ok(ModbusResponse::Exception(_, code)) => {
                                                    let _ = response_tx.send(Err(ModbusToolError::ModbusException(code.to_u8())));
                                                }
                                                Ok(_) => {
                                                    let _ = response_tx.send(Ok(crate::models::WriteResult {
                                                        timestamp_ms: chrono::Utc::now().timestamp_millis(),
                                                        request_frame: frame,
                                                        response_frame: full_frame,
                                                        elapsed_ms: elapsed,
                                                    }));
                                                }
                                                Err(e) => { let _ = response_tx.send(Err(e)); }
                                            }
                                        }
                                        Err(e) => { let _ = response_tx.send(Err(e)); }
                                    }
                                }
                                Ok(Err(_)) => { let _ = response_tx.send(Err(ModbusToolError::SerialPortNotFound)); break; }
                                Err(_) => { let _ = response_tx.send(Err(ModbusToolError::ResponseTimeout)); }
                            }
                        }
                        Some(WorkerCommand::Shutdown) => break,
                        None => break,
                    }
                }
            }
        }
    }

    fn open_port(&self) -> Result<SerialStream, ModbusToolError> {
        tokio_serial::new(&self.config.port_name, self.config.baud_rate)
            .data_bits(match self.config.data_bits {
                5 => tokio_serial::DataBits::Five,
                6 => tokio_serial::DataBits::Six,
                7 => tokio_serial::DataBits::Seven,
                _ => tokio_serial::DataBits::Eight,
            })
            .stop_bits(match self.config.stop_bits {
                2 => tokio_serial::StopBits::Two,
                _ => tokio_serial::StopBits::One,
            })
            .parity(match self.config.parity.as_str() {
                "odd" => tokio_serial::Parity::Odd,
                "even" => tokio_serial::Parity::Even,
                _ => tokio_serial::Parity::None,
            })
            .open_native_async()
            .map_err(|e| ModbusToolError::SerialOpenFailed(e.to_string()))
    }
}
