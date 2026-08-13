<script setup lang="ts">
import { useConnectionStore } from './stores/connection'
import ConnectionPanel from './components/ConnectionPanel.vue'
import ReadValuePanel from './components/ReadValuePanel.vue'
import WriteValuePanel from './components/WriteValuePanel.vue'
import LogsPanel from './components/LogsPanel.vue'

const connectionStore = useConnectionStore()
</script>

<template>
  <div class="app-layout">
    <aside class="sidebar">
      <div class="brand">
        <h1>Modbus Tools</h1>
      </div>
      <ConnectionPanel />
    </aside>
    
    <main class="main-content">
      <div v-if="connectionStore.status === 'Connected'" class="workspace">
        <div class="panels-row">
          <ReadValuePanel class="flex-1" />
          <WriteValuePanel class="flex-1" />
        </div>
        <LogsPanel />
      </div>
      <div v-else class="empty-state">
        <p>Please connect to a device to start reading data.</p>
      </div>
    </main>
  </div>
</template>

<style scoped>
.app-layout {
  display: flex;
  width: 100%;
  height: 100%;
}

.sidebar {
  width: 320px;
  background-color: var(--bg-secondary);
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
}

.brand {
  padding: 1.5rem;
  border-bottom: 1px solid var(--border);
}

.brand h1 {
  font-size: 1.25rem;
  font-weight: 700;
  color: var(--text-primary);
  background: linear-gradient(to right, var(--accent), #60a5fa);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.main-content {
  flex: 1;
  background-color: var(--bg-primary);
  overflow-y: auto;
  padding: 2rem;
}

.workspace {
  max-width: 1400px;
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.panels-row {
  display: flex;
  gap: 2rem;
  align-items: flex-start;
}

.flex-1 {
  flex: 1;
  min-width: 0;
}

.empty-state {
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-secondary);
  font-size: 1.125rem;
}
</style>