use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::time::timeout;
use tokio_util::sync::CancellationToken;
use socket2::{Socket, TcpKeepalive};
use crate::models::{TcpConfig, ReadResult, ParsedValue};
use crate::error::ModbusToolError;
use crate::connection::commands::WorkerCommand;
use crate::protocol::tcp_adu::{encode_tcp_request, decode_tcp_response_header};
use crate::protocol::pdu::{decode_response, ModbusResponse};
use crate::parser::data_type::parse_data;
use crate::parser::transform::apply_transform;
use bytes::BytesMut;

pub struct TcpWorker {
    config: TcpConfig,
    rx: mpsc::Receiver<WorkerCommand>,
    cancel_token: CancellationToken,
    transaction_id: u16,
}

impl TcpWorker {
    pub fn new(
        config: TcpConfig,
        rx: mpsc::Receiver<WorkerCommand>,
        cancel_token: CancellationToken,
    ) -> Self {
        Self {
            config,
            rx,
            cancel_token,
            transaction_id: 1,
        }
    }

    pub async fn run(mut self) {
        let mut stream = match self.connect().await {
            Ok(s) => s,
            Err(_) => return, // Handle error, maybe emit event
        };

        let mut buf = BytesMut::with_capacity(1024);

        loop {
            tokio::select! {
                _ = self.cancel_token.cancelled() => {
                    break;
                }
                cmd = self.rx.recv() => {
                    match cmd {
                        Some(WorkerCommand::Read { request, response_tx }) => {
                            self.transaction_id = self.transaction_id.wrapping_add(1);
                            
                            let pdu = match request.function {
                                1 => crate::protocol::pdu::ModbusRequest::ReadCoils { address: request.address, quantity: request.data_type.register_count() }.encode(),
                                2 => crate::protocol::pdu::ModbusRequest::ReadDiscreteInputs { address: request.address, quantity: request.data_type.register_count() }.encode(),
                                3 => crate::protocol::pdu::ModbusRequest::ReadHoldingRegisters { address: request.address, quantity: request.data_type.register_count() }.encode(),
                                4 => crate::protocol::pdu::ModbusRequest::ReadInputRegisters { address: request.address, quantity: request.data_type.register_count() }.encode(),
                                _ => {
                                    let _ = response_tx.send(Err(ModbusToolError::UnsupportedFunctionCode(request.function)));
                                    continue;
                                }
                            };
                            
                            let frame = encode_tcp_request(self.transaction_id, self.config.unit_id, &pdu);
                            let start_time = chrono::Utc::now();
                            
                            if stream.write_all(&frame).await.is_err() {
                                let _ = response_tx.send(Err(ModbusToolError::TcpDisconnected));
                                break;
                            }

                            // Read response

                            let read_timeout = Duration::from_millis(self.config.response_timeout_ms);
                            
                            let read_result = timeout(read_timeout, async {
                                loop {
                                    if let Ok(Some((header, total_len))) = decode_tcp_response_header(&mut buf) {
                                        if header.transaction_id == self.transaction_id {
                                            let full_frame = buf.split_to(total_len).to_vec();
                                            return Ok(full_frame);
                                        } else {
                                            buf.split_to(total_len); // Drop unexpected frame
                                            continue;
                                        }
                                    }
                                    
                                    let mut temp_buf = [0u8; 1024];
                                    match stream.read(&mut temp_buf).await {
                                        Ok(0) => return Err(ModbusToolError::TcpDisconnected),
                                        Ok(n) => buf.extend_from_slice(&temp_buf[..n]),
                                        Err(_) => return Err(ModbusToolError::TcpDisconnected),
                                    }
                                }
                            }).await;

                            let elapsed = chrono::Utc::now().signed_duration_since(start_time).num_milliseconds() as u64;

                            match read_result {
                                Ok(Ok(full_frame)) => {
                                    // Parse PDU
                                    let pdu_data = &full_frame[7..];
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
                                Ok(Err(e)) => { let _ = response_tx.send(Err(e)); break; }
                                Err(_) => { let _ = response_tx.send(Err(ModbusToolError::ResponseTimeout)); }
                            }
                        }
                        Some(WorkerCommand::Write { request, response_tx }) => {
                            self.transaction_id = self.transaction_id.wrapping_add(1);
                            
                            let pdu = match request.function {
                                5 => crate::protocol::pdu::ModbusRequest::WriteSingleCoil { address: request.address, value: request.values[0] != 0 }.encode(),
                                6 => crate::protocol::pdu::ModbusRequest::WriteSingleRegister { address: request.address, value: request.values[0] }.encode(),
                                15 => {
                                    let mut packed = vec![0u8; (request.values.len() + 7) / 8];
                                    for (i, v) in request.values.iter().enumerate() {
                                        if *v != 0 {
                                            packed[i / 8] |= 1 << (i % 8);
                                        }
                                    }
                                    crate::protocol::pdu::ModbusRequest::WriteMultipleCoils { address: request.address, quantity: request.values.len() as u16, values: packed }.encode()
                                },
                                16 => crate::protocol::pdu::ModbusRequest::WriteMultipleRegisters { address: request.address, quantity: request.values.len() as u16, values: request.values.clone() }.encode(),
                                _ => {
                                    let _ = response_tx.send(Err(ModbusToolError::UnsupportedFunctionCode(request.function)));
                                    continue;
                                }
                            };
                            
                            let frame = encode_tcp_request(self.transaction_id, self.config.unit_id, &pdu);
                            let start_time = chrono::Utc::now();
                            
                            if stream.write_all(&frame).await.is_err() {
                                let _ = response_tx.send(Err(ModbusToolError::TcpDisconnected));
                                break;
                            }

                            // Read response
                            let read_timeout = Duration::from_millis(self.config.response_timeout_ms);
                            let read_result = timeout(read_timeout, async {
                                loop {
                                    if let Ok(Some((header, total_len))) = decode_tcp_response_header(&mut buf) {
                                        if header.transaction_id == self.transaction_id {
                                            let full_frame = buf.split_to(total_len).to_vec();
                                            return Ok(full_frame);
                                        } else {
                                            let _ = buf.split_to(total_len); // Drop unexpected frame
                                            continue;
                                        }
                                    }
                                    let mut temp_buf = [0u8; 1024];
                                    match stream.read(&mut temp_buf).await {
                                        Ok(0) => return Err(ModbusToolError::TcpDisconnected),
                                        Ok(n) => buf.extend_from_slice(&temp_buf[..n]),
                                        Err(_) => return Err(ModbusToolError::TcpDisconnected),
                                    }
                                }
                            }).await;

                            let elapsed = chrono::Utc::now().signed_duration_since(start_time).num_milliseconds() as u64;

                            match read_result {
                                Ok(Ok(full_frame)) => {
                                    let pdu_data = &full_frame[7..];
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
                                Ok(Err(e)) => { let _ = response_tx.send(Err(e)); break; }
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

    async fn connect(&self) -> Result<TcpStream, ModbusToolError> {
        let addr = format!("{}:{}", self.config.host, self.config.port);
        let stream = timeout(
            Duration::from_millis(self.config.connect_timeout_ms),
            TcpStream::connect(&addr),
        )
        .await
        .map_err(|_| ModbusToolError::TcpConnectTimeout)?
        .map_err(|_| ModbusToolError::TcpDisconnected)?;

        // Apply TCP Keep-Alive
        let socket = Socket::try_from(stream.into_std().unwrap()).unwrap();
        let keepalive = TcpKeepalive::new()
            .with_time(Duration::from_secs(3))
            .with_interval(Duration::from_secs(1))
            .with_retries(3);
        let _ = socket.set_tcp_keepalive(&keepalive);
        Ok(TcpStream::from_std(socket.into()).unwrap())
    }
}
