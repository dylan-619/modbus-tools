<script setup lang="ts">
import { ref } from 'vue'
import { useConnectionStore } from './stores/connection'
import ConnectionPanel from './components/ConnectionPanel.vue'
import ReadValuePanel from './components/ReadValuePanel.vue'
import WriteValuePanel from './components/WriteValuePanel.vue'
import LogsPanel from './components/LogsPanel.vue'

const connectionStore = useConnectionStore()
const activeTab = ref<'read' | 'write'>('read')
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
        <div class="workspace-tabs">
          <button 
            class="tab-btn" 
            :class="{ active: activeTab === 'read' }" 
            @click="activeTab = 'read'"
          >
            Read Data
          </button>
          <button 
            class="tab-btn" 
            :class="{ active: activeTab === 'write' }" 
            @click="activeTab = 'write'"
          >
            Write Data
          </button>
        </div>
        
        <div class="active-panel">
          <ReadValuePanel v-if="activeTab === 'read'" class="flex-1" />
          <WriteValuePanel v-if="activeTab === 'write'" class="flex-1" />
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
  width: 250px;
  background-color: var(--bg-secondary);
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
}

.brand {
  padding: 12px;
  border-bottom: 1px solid var(--border);
}

.brand h1 {
  font-size: 16px;
  font-weight: 700;
  color: var(--text-primary);
  background: linear-gradient(to right, var(--accent), #60a5fa);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.main-content {
  flex: 1;
  background-color: var(--bg-primary);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  padding: 12px;
}

.workspace {
  display: flex;
  flex-direction: column;
  gap: 12px;
  flex: 1;
  overflow: hidden;
}

.workspace-tabs {
  display: flex;
  background-color: var(--bg-primary);
  border-bottom: 1px solid var(--border);
  gap: 2px;
}

.tab-btn {
  padding: 8px 16px;
  background: transparent;
  border: none;
  color: var(--text-secondary);
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  border-bottom: 2px solid transparent;
}

.tab-btn:hover {
  color: var(--text-primary);
}

.tab-btn.active {
  color: var(--accent);
  border-bottom: 2px solid var(--accent);
}

.active-panel {
  display: flex;
  flex: 0 0 auto;
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