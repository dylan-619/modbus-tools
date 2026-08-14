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
        <label>
          Address
          <label class="checkbox-label" style="float: right; font-size: 0.8rem; font-weight: normal; cursor: pointer;">
            <input type="checkbox" :checked="store.addressMode === 'PLC'" @change="e => store.addressMode = (e.target as HTMLInputElement).checked ? 'PLC' : 'Protocol'" />
            PLC Mode (e.g. 40063)
          </label>
        </label>
        <input type="text" v-model="store.request.address" class="input" placeholder="e.g. 62 or 40063" />
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
  grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
  gap: 8px;
  margin-bottom: 12px;
}

.actions {
  display: flex;
  justify-content: flex-end;
  margin-bottom: 12px;
  padding-bottom: 12px;
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
  padding: 12px;
  border: 1px solid var(--border);
}

.value {
  font-size: 16px;
  font-weight: 700;
  display: flex;
  align-items: baseline;
  gap: 8px;
  margin-bottom: 8px;
}

.value .label {
  font-size: 13px;
  color: var(--text-secondary);
  font-weight: 500;
}

.data {
  color: var(--text-primary);
}
.data.success {
  color: var(--success);
}

.checkbox-label {
  display: inline-flex;
  align-items: center;
  gap: 0.3rem;
  color: var(--text-secondary);
}
.checkbox-label:hover {
  color: var(--text-primary);
}

.meta {
  display: flex;
  gap: 8px;
  color: var(--text-secondary);
  font-size: 11px;
  margin-bottom: 8px;
}
</style>
