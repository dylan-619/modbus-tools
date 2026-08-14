import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { WriteRequest, WriteResult } from '../types/modbus'
import { useConnectionStore } from './connection'
import { useLogsStore } from './logs'
import { parseModbusAddress, type AddressMode } from '../utils/address'

export const useWriterStore = defineStore('writer', () => {
  const connectionStore = useConnectionStore()
  const logsStore = useLogsStore()
  
  const addressMode = ref<AddressMode>('Protocol')
  
  const request = ref<Omit<WriteRequest, 'connection_id'>>({
    function: 6,
    address: 0,
    values: [0]
  })
  
  const lastResult = ref<WriteResult | null>(null)
  const isWriting = ref(false)

  async function writeOnce() {
    if (!connectionStore.connectionId) return
    
    isWriting.value = true
    try {
      const parsedAddress = parseModbusAddress(request.value.address, addressMode.value, request.value.function)

      const fullRequest: WriteRequest = {
        connection_id: connectionStore.connectionId,
        function: request.value.function,
        address: parsedAddress,
        values: request.value.values
      }
      
      const res = await invoke<WriteResult>('write_value', { request: fullRequest })
      lastResult.value = res

      // Log TX
      logsStore.addLog({
        id: Date.now(),
        timestamp_ms: res.timestamp_ms - res.elapsed_ms,
        direction: 'Tx',
        transport: connectionStore.transportType,
        connection_id: connectionStore.connectionId,
        slave_id: connectionStore.transportType === 'tcp' ? connectionStore.tcpConfig.unit_id : connectionStore.rtuConfig.slave_id,
        function_code: request.value.function,
        data: res.request_frame,
        message: null
      })

      // Log RX
      logsStore.addLog({
        id: Date.now() + 1,
        timestamp_ms: res.timestamp_ms,
        direction: 'Rx',
        transport: connectionStore.transportType,
        connection_id: connectionStore.connectionId,
        slave_id: connectionStore.transportType === 'tcp' ? connectionStore.tcpConfig.unit_id : connectionStore.rtuConfig.slave_id,
        function_code: request.value.function,
        data: res.response_frame,
        message: null
      })

    } catch (e: any) {
      console.error('Write failed:', e)
      logsStore.addLog({
        id: Date.now(),
        timestamp_ms: Date.now(),
        direction: 'Error',
        transport: connectionStore.transportType,
        connection_id: connectionStore.connectionId,
        slave_id: connectionStore.transportType === 'tcp' ? connectionStore.tcpConfig.unit_id : connectionStore.rtuConfig.slave_id,
        function_code: request.value.function,
        data: [],
        message: String(e)
      })
    } finally {
      isWriting.value = false
    }
  }

  return {
    request,
    addressMode,
    lastResult,
    isWriting,
    writeOnce
  }
})
