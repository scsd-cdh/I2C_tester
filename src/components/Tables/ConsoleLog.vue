<template>
  <div class="log-container">
    <div v-for="(log, index) in logs.slice().reverse()" :key="index"
      :class="['log-entry', { success: log.success, failure: !log.success }]">
      <div class="log-method">{{ log.title }}</div>
      <div class="log-message">{{ log.message }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Log } from "../../types/types"


const props = defineProps<{
  logs: Log[]
}>();
</script>

<style scoped>
.log-container {
  margin-left: auto;
  margin-right: auto;
  padding: 1rem;
  border: 1px solid #ccc;
  border-radius: 4px;
  max-height: 60vh;
  max-width: 80vw;
  overflow-y: auto;
  background-color: #f9f9f9;
}

.log-entry {
  padding: 0.5rem;
  border-bottom: 1px solid #ddd;
}

.log-entry:last-child {
  border-bottom: none;
}

.log-method {
  font-weight: bold;
}

.success {
  color: green;
}

.failure {
  color: red;
}
</style>
