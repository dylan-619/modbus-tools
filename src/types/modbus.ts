export type ConnectionStatus = 'Disconnected' | 'Connecting' | 'Connected' | 'Reconnecting' | 'Failed'

export type TransportType = 'tcp' | 'rtu'

export interface TcpConfig {
  host: string
  port: number
  unit_id: number
  connect_timeout_ms: number
  response_timeout_ms: number
}

export interface RtuConfig {
  port_name: string
  slave_id: number
  baud_rate: number
  data_bits: number
  stop_bits: number
  parity: 'none' | 'odd' | 'even'
  response_timeout_ms: number
  inter_request_delay_ms: number
}

export interface TransportConfig {
  type: TransportType
  tcp?: TcpConfig
  rtu?: RtuConfig
}

export interface SerialPortInfo {
  name: string
  port_type: string
}

export type DataType = 
  | 'Bool' 
  | 'UInt16' 
  | 'Int16' 
  | 'UInt32' 
  | 'Int32' 
  | 'Float32' 
  | 'UInt64' 
  | 'Int64' 
  | 'Float64' 
  | { Ascii: { registers: number } }
  | { Raw: { registers: number } }

export type ByteLayout = 
  | 'Ab' | 'Ba' 
  | 'Abcd' | 'Cdab' | 'Badc' | 'Dcba'
  | 'Abcdefgh' | 'Ghefcdab' | 'Badcfehg' | 'Hgfedcba'

export interface ValueTransform {
  scale: number
  offset: number
  decimals: number | null
  unit: string | null
}

export interface ReadRequest {
  connection_id: string
  function: number
  address: number
  data_type: DataType
  layout: ByteLayout
  transform: ValueTransform
}

export interface ReadResult {
  timestamp_ms: number
  raw_value: any
  transformed_value: number | null
  display_value: string
  registers: number[]
  data_bytes: number[]
  request_frame: number[]
  response_frame: number[]
  elapsed_ms: number
}

export interface FrameLog {
  id: number
  timestamp_ms: number
  direction: 'Tx' | 'Rx' | 'Error'
  transport: TransportType
  connection_id: string
  slave_id: number
  function_code: number
  data: number[]
  message: string | null
}

export interface WriteRequest {
  connection_id: string
  function: number
  address: number
  values: number[]
}

export interface WriteResult {
  timestamp_ms: number
  request_frame: number[]
  response_frame: number[]
  elapsed_ms: number
}
