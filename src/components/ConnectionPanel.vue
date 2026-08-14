<script setup lang="ts">
import { useConnectionStore } from '../stores/connection'
import { onMounted } from 'vue'

const store = useConnectionStore()

onMounted(() => {
  store.loadPorts()
})
</script>

<template>
  <div class="connection-panel">
    <div class="tabs">
      <button 
        class="tab" 
        :class="{ active: store.transportType === 'tcp' }"
        @click="store.transportType = 'tcp'"
        :disabled="store.status === 'Connecting' || store.status === 'Connected'"
      >
        TCP
      </button>
      <button 
        class="tab" 
        :class="{ active: store.transportType === 'rtu' }"
        @click="store.transportType = 'rtu'"
        :disabled="store.status === 'Connecting' || store.status === 'Connected'"
      >
        RTU
      </button>
    </div>

    <div class="settings">
      <!-- TCP Settings -->
      <div v-if="store.transportType === 'tcp'" class="form-group">
        <div class="input-group">
          <label>Host</label>
          <input type="text" v-model="store.tcpConfig.host" class="input" :disabled="store.status === 'Connecting' || store.status === 'Connected'" />
        </div>
        <div class="input-group">
          <label>Port</label>
          <input type="number" v-model.number="store.tcpConfig.port" class="input" :disabled="store.status === 'Connecting' || store.status === 'Connected'" />
        </div>
        <div class="input-group">
          <label>Unit ID</label>
          <input type="number" v-model.number="store.tcpConfig.unit_id" class="input" :disabled="store.status === 'Connecting' || store.status === 'Connected'" />
        </div>
      </div>

      <!-- RTU Settings -->
      <div v-if="store.transportType === 'rtu'" class="form-group">
        <div class="input-group">
          <label>Serial Port</label>
          <select v-model="store.rtuConfig.port_name" class="select" :disabled="store.status === 'Connecting' || store.status === 'Connected'">
            <option value="" disabled>Select Port</option>
            <option v-for="port in store.availablePorts" :key="port.name" :value="port.name">
              {{ port.name }}
            </option>
          </select>
        </div>
        <div class="input-group">
          <label>Baud Rate</label>
          <select v-model.number="store.rtuConfig.baud_rate" class="select" :disabled="store.status === 'Connecting' || store.status === 'Connected'">
            <option :value="9600">9600</option>
            <option :value="19200">19200</option>
            <option :value="38400">38400</option>
            <option :value="115200">115200</option>
          </select>
        </div>
        <div class="input-group">
          <label>Slave ID</label>
          <input type="number" v-model.number="store.rtuConfig.slave_id" class="input" :disabled="store.status === 'Connecting' || store.status === 'Connected'" />
        </div>
      </div>
    </div>

    <div class="actions">
      <button 
        v-if="store.status === 'Disconnected' || store.status === 'Failed'" 
        class="btn btn-primary connect-btn"
        @click="store.connect"
      >
        Connect
      </button>
      <button 
        v-else 
        class="btn btn-danger connect-btn"
        @click="store.disconnect"
      >
        {{ store.status === 'Connecting' ? 'Connecting...' : 'Disconnect' }}
      </button>
    </div>

    <div v-if="store.errorMessage" class="error-msg">
      {{ store.errorMessage }}
    </div>
  </div>
</template>

<style scoped>
.connection-panel {
  padding: 12px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.tabs {
  display: flex;
  background-color: var(--bg-primary);
  border-radius: var(--radius-md);
  padding: 0.25rem;
}

.tab {
  flex: 1;
  padding: 4px 8px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  font-weight: 500;
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: all 0.2s;
  font-size: 13px;
}

.tab.active {
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
  box-shadow: var(--shadow-sm);
}

.tab:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.form-group {
  display: flex;
  flex-direction: column;
}

.connect-btn {
  width: 100%;
  padding: 6px;
  font-size: 13px;
}

.error-msg {
  color: var(--error);
  font-size: 12px;
  padding: 8px;
  background-color: rgba(239, 68, 68, 0.1);
  border-radius: var(--radius-md);
  border: 1px solid rgba(239, 68, 68, 0.2);
}
</style>
