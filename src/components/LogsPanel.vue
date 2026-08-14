<script setup lang="ts">
import { useLogsStore } from '../stores/logs'
import { ref } from 'vue'

const store = useLogsStore()
const autoScroll = ref(true)

function formatHex(data: number[]) {
  return data.map(b => b.toString(16).padStart(2, '0').toUpperCase()).join(' ')
}
</script>

<template>
  <div class="panel logs-panel">
    <div class="header">
      <h2 class="heading">Traffic Logs</h2>
      <div class="controls">
        <label class="checkbox">
          <input type="checkbox" v-model="autoScroll" /> Auto-scroll
        </label>
        <button class="btn" @click="store.isPaused = !store.isPaused">
          {{ store.isPaused ? 'Resume' : 'Pause' }}
        </button>
        <button class="btn btn-danger" @click="store.clearLogs">Clear</button>
      </div>
    </div>
    
    <div class="logs-container" :class="{ 'auto-scroll': autoScroll }">
      <table class="logs-table">
        <thead>
          <tr>
            <th>Time</th>
            <th>Dir</th>
            <th>Type</th>
            <th>ID</th>
            <th>FC</th>
            <th>Data</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="log in store.entries" :key="log.id" :class="log.direction.toLowerCase()">
            <td>{{ new Date(log.timestamp_ms).toLocaleTimeString() }}.{{ String(log.timestamp_ms % 1000).padStart(3, '0') }}</td>
            <td><span class="badge" :class="log.direction.toLowerCase()">{{ log.direction }}</span></td>
            <td>{{ log.transport.toUpperCase() }}</td>
            <td>{{ log.slave_id }}</td>
            <td>{{ log.function_code }}</td>
            <td class="data-col">{{ log.message || formatHex(log.data) }}</td>
          </tr>
        </tbody>
      </table>
      <div v-if="store.entries.length === 0" class="empty-logs">
        No traffic logs yet.
      </div>
    </div>
  </div>
</template>

<style scoped>
.logs-panel {
  display: flex;
  flex-direction: column;
  flex: 1;
  min-height: 200px;
  overflow: hidden;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.controls {
  display: flex;
  gap: 8px;
  align-items: center;
}

.checkbox {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 12px;
  color: var(--text-secondary);
  cursor: pointer;
}

.logs-container {
  flex: 1;
  overflow-y: auto;
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  background-color: var(--bg-primary);
}

.auto-scroll {
  scroll-behavior: smooth;
}

.logs-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 12px;
}

.logs-table th {
  position: sticky;
  top: 0;
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
  text-align: left;
  padding: 4px 8px;
  font-weight: 600;
  z-index: 10;
}

.logs-table td {
  padding: 4px 8px;
  border-bottom: 1px solid var(--border);
  color: var(--text-secondary);
}

.logs-table tr:hover td {
  background-color: rgba(255, 255, 255, 0.05);
}

.data-col {
  font-family: monospace;
  word-break: break-all;
}

.badge {
  padding: 2px 4px;
  border-radius: var(--radius-sm);
  font-size: 11px;
  font-weight: 600;
}

.badge.tx {
  background-color: rgba(59, 130, 246, 0.2);
  color: #60a5fa;
}

.badge.rx {
  background-color: rgba(16, 185, 129, 0.2);
  color: #34d399;
}

.badge.error {
  background-color: rgba(239, 68, 68, 0.2);
  color: #f87171;
}

.empty-logs {
  padding: 2rem;
  text-align: center;
  color: var(--text-secondary);
}
</style>
