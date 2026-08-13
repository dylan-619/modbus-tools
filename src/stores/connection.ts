import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { ConnectionStatus, TransportType, TcpConfig, RtuConfig, SerialPortInfo } from '../types/modbus'

export const useConnectionStore = defineStore('connection', () => {
  const transportType = ref<TransportType>('tcp')
  const status = ref<ConnectionStatus>('Disconnected')
  const connectionId = ref<string | null>(null)
  const errorMessage = ref<string | null>(null)
  const availablePorts = ref<SerialPortInfo[]>([])

  const tcpConfig = ref<TcpConfig>({
    host: '127.0.0.1',
    port: 502,
    unit_id: 1,
    connect_timeout_ms: 2000,
    response_timeout_ms: 1000,
  })

  const rtuConfig = ref<RtuConfig>({
    port_name: '',
    slave_id: 1,
    baud_rate: 9600,
    data_bits: 8,
    stop_bits: 1,
    parity: 'none',
    response_timeout_ms: 1000,
    inter_request_delay_ms: 10,
  })

  async function loadPorts() {
    try {
      availablePorts.value = await invoke<SerialPortInfo[]>('list_serial_ports')
    } catch (e) {
      console.error('Failed to load serial ports:', e)
    }
  }

  async function connect() {
    status.value = 'Connecting'
    errorMessage.value = null
    const id = crypto.randomUUID()
    connectionId.value = id

    try {
      const config = transportType.value === 'tcp' 
        ? { type: 'tcp', ...tcpConfig.value } 
        : { type: 'rtu', ...rtuConfig.value }

      await invoke('connect_modbus', { connectionId: id, config })
      status.value = 'Connected'
    } catch (e: any) {
      status.value = 'Failed'
      errorMessage.value = String(e)
    }
  }

  async function disconnect() {
    if (!connectionId.value) return
    try {
      await invoke('disconnect_modbus', { connectionId: connectionId.value })
    } finally {
      status.value = 'Disconnected'
      connectionId.value = null
    }
  }

  return {
    transportType,
    status,
    connectionId,
    errorMessage,
    availablePorts,
    tcpConfig,
    rtuConfig,
    loadPorts,
    connect,
    disconnect
  }
})
