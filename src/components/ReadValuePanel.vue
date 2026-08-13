<script setup lang="ts">
import { useReaderStore } from '../stores/reader'

const store = useReaderStore()

const functionOptions = [
  { value: 1, label: '01 Read Coils' },
  { value: 2, label: '02 Read Discrete Inputs' },
  { value: 3, label: '03 Read Holding Registers' },
  { value: 4, label: '04 Read Input Registers' },
]

const dataTypeOptions = [
  'Bool', 'UInt16', 'Int16', 'UInt32', 'Int32', 
  'Float32', 'UInt64', 'Int64', 'Float64'
]

const layoutOptions = [
  { value: 'Ab', label: 'AB' },
  { value: 'Ba', label: 'BA' },
  { value: 'Abcd', label: 'ABCD' },
  { value: 'Cdab', label: 'CDAB' },
  { value: 'Badc', label: 'BADC' },
  { value: 'Dcba', label: 'DCBA' },
]
</script>

<template>
  <div class="panel">
    <h2 class="heading">Read Data</h2>
    
    <div class="grid-controls">
      <div class="input-group">
        <label>Function Code</label>
        <select v-model.number="store.request.function" class="select">
          <option v-for="opt in functionOptions" :key="opt.value" :value="opt.value">
            {{ opt.label }}
          </option>
        </select>
      </div>
      
      <div class="input-group">
        <label>Address</label>
        <input type="number" v-model.number="store.request.address" class="input" min="0" max="65535" />
      </div>

      <div class="input-group">
        <label>Data Type</label>
        <select v-model="store.request.data_type" class="select">
          <option v-for="type in dataTypeOptions" :key="type" :value="type">
            {{ type }}
          </option>
        </select>
      </div>

      <div class="input-group">
        <label>Byte Layout</label>
        <select v-model="store.request.layout" class="select">
          <option v-for="opt in layoutOptions" :key="opt.value" :value="opt.value">
            {{ opt.label }}
          </option>
        </select>
      </div>
    </div>

    <div class="actions">
      <button class="btn btn-primary" @click="store.readOnce">Read Once</button>
      
      <div class="polling-controls">
        <button 
          class="btn" 
          :class="store.isPolling ? 'btn-danger' : 'btn-success'"
          @click="store.togglePolling"
        >
          {{ store.isPolling ? 'Stop Polling' : 'Auto Poll' }}
        </button>
        <div v-if="!store.isPolling" class="input-group inline">
          <input type="number" v-model.number="store.pollingIntervalMs" class="input small-input" min="100" />
          <span>ms</span>
        </div>
      </div>
    </div>

    <div v-if="store.lastResult" class="result-display">
      <div class="value">
        <span class="label">Result:</span>
        <span class="data">{{ store.lastResult.display_value }}</span>
      </div>
      <div class="meta">
        <span class="time">{{ new Date(store.lastResult.timestamp_ms).toLocaleTimeString() }}</span>
        <span class="elapsed">{{ store.lastResult.elapsed_ms }}ms</span>
      </div>
      <div class="raw-frames">
        <div><strong>TX:</strong> {{ store.lastResult.request_frame.map(b => b.toString(16).padStart(2, '0')).join(' ') }}</div>
        <div><strong>RX:</strong> {{ store.lastResult.response_frame.map(b => b.toString(16).padStart(2, '0')).join(' ') }}</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.grid-controls {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
  padding-bottom: 1.5rem;
  border-bottom: 1px solid var(--border);
}

.polling-controls {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.inline {
  flex-direction: row;
  align-items: center;
  margin-bottom: 0;
}

.small-input {
  width: 80px;
  padding: 0.25rem 0.5rem;
}

.result-display {
  background-color: var(--bg-primary);
  border-radius: var(--radius-md);
  padding: 1.5rem;
  border: 1px solid var(--border);
}

.value {
  font-size: 2rem;
  font-weight: 700;
  display: flex;
  align-items: baseline;
  gap: 1rem;
  margin-bottom: 1rem;
}

.value .label {
  font-size: 1rem;
  color: var(--text-secondary);
  font-weight: 500;
}

.value .data {
  color: var(--success);
}

.meta {
  display: flex;
  gap: 1rem;
  color: var(--text-secondary);
  font-size: 0.875rem;
  margin-bottom: 1rem;
}

.raw-frames {
  font-family: monospace;
  font-size: 0.875rem;
  color: var(--text-secondary);
  background-color: rgba(0,0,0,0.2);
  padding: 1rem;
  border-radius: var(--radius-sm);
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.raw-frames strong {
  color: var(--text-primary);
}
</style>
