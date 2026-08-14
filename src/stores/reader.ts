import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { ReadRequest, ReadResult } from '../types/modbus'
import { useConnectionStore } from './connection'
import { useLogsStore } from './logs'
import { parseModbusAddress, type AddressMode } from '../utils/address'

export const useReaderStore = defineStore('reader', () => {
  const connectionStore = useConnectionStore()
  const logsStore = useLogsStore()
  
  const addressMode = ref<AddressMode>('Protocol')
  
  const request = ref<Omit<ReadRequest, 'connection_id'>>({
    function: 3,
    address: 0,
    data_type: 'UInt16',
    layout: 'Ab',
    transform: { scale: 1.0, offset: 0.0, decimals: null, unit: null }
  })
  
  const lastResult = ref<ReadResult | null>(null)
  const isPolling = ref(false)
  const pollingIntervalMs = ref(1000)
  let pollIntervalId: number | null = null

  async function readOnce() {
    if (!connectionStore.connectionId) return
    
    try {
      const parsedAddress = parseModbusAddress(request.value.address, addressMode.value, request.value.function)
      
      const fullRequest: ReadRequest = {
        ...request.value,
        address: parsedAddress,
        connection_id: connectionStore.connectionId
      }
      
      const res = await invoke<ReadResult>('read_value', { request: fullRequest })
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
      console.error('Read failed:', e)
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
    }
  }

  function togglePolling() {
    if (isPolling.value) {
      stopPolling()
    } else {
      startPolling()
    }
  }

  function startPolling() {
    isPolling.value = true
    readOnce()
    pollIntervalId = window.setInterval(readOnce, pollingIntervalMs.value)
  }

  function stopPolling() {
    isPolling.value = false
    if (pollIntervalId !== null) {
      clearInterval(pollIntervalId)
      pollIntervalId = null
    }
  }

  return {
    request,
    addressMode,
    lastResult,
    isPolling,
    pollingIntervalMs,
    readOnce,
    togglePolling,
    startPolling,
    stopPolling
  }
})
