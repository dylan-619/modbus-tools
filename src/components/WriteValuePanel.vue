<script setup lang="ts">
import { useWriterStore } from '../stores/writer'
import { computed } from 'vue'

const store = useWriterStore()

const functionOptions = [
  { value: 5, label: '05 Write Single Coil' },
  { value: 6, label: '06 Write Single Register' },
  { value: 15, label: '15 Write Multiple Coils' },
  { value: 16, label: '16 Write Multiple Registers' },
]

const valueInput = computed({
  get: () => store.request.values.join(', '),
  set: (val: string) => {
    store.request.values = val.split(',').map(s => parseInt(s.trim(), 10)).filter(n => !isNaN(n))
  }
})
</script>

<template>
  <div class="panel">
    <h2 class="heading">Write Data</h2>
    
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
        <label>Values (comma separated for multiple)</label>
        <input type="text" v-model="valueInput" class="input" placeholder="e.g. 1, 0, 1 or 256, 12" />
      </div>
    </div>

    <div class="actions">
      <button 
        class="btn btn-warning" 
        :disabled="store.isWriting || store.request.values.length === 0"
        @click="store.writeOnce"
      >
        {{ store.isWriting ? 'Writing...' : 'Write Data' }}
      </button>
    </div>

    <div v-if="store.lastResult" class="result-display">
      <div class="value">
        <span class="label">Status:</span>
        <span class="data success">Success</span>
      </div>
      <div class="meta">
        <span class="time">{{ new Date(store.lastResult.timestamp_ms).toLocaleTimeString() }}</span>
        <span class="elapsed">{{ store.lastResult.elapsed_ms }}ms</span>
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
  justify-content: flex-end;
  margin-bottom: 2rem;
  padding-bottom: 1.5rem;
  border-bottom: 1px solid var(--border);
}

.btn-warning {
  background-color: var(--warning);
  color: white;
}

.btn-warning:hover:not(:disabled) {
  filter: brightness(1.1);
  box-shadow: 0 0 10px rgba(245, 158, 11, 0.5);
}

.result-display {
  background-color: var(--bg-primary);
  border-radius: var(--radius-md);
  padding: 1.5rem;
  border: 1px solid var(--border);
}

.value {
  font-size: 1.5rem;
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

.value .data.success {
  color: var(--success);
}

.meta {
  display: flex;
  gap: 1rem;
  color: var(--text-secondary);
  font-size: 0.875rem;
}
</style>
