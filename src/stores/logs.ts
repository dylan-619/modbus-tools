import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { FrameLog } from '../types/modbus'

export const useLogsStore = defineStore('logs', () => {
  const entries = ref<FrameLog[]>([])
  const maxEntries = ref(10000)
  const isPaused = ref(false)

  function addLog(log: FrameLog) {
    if (isPaused.value) return
    entries.value.push(log)
    if (entries.value.length > maxEntries.value) {
      entries.value.shift()
    }
  }

  function clearLogs() {
    entries.value = []
  }

  return {
    entries,
    maxEntries,
    isPaused,
    addLog,
    clearLogs
  }
})
