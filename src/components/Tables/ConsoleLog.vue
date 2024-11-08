<template>
  <div class="mx-auto p-4 border border-gray-300 rounded-md max-h-[60vh] max-w-[80vw] overflow-y-auto bg-gray-100">
    <div v-for="(log, index) in logs.slice().reverse()" :key="index" :class="[
      'py-2 border-b border-gray-200 last:border-none',
      log.success ? 'text-green-600' : 'text-red-600'
    ]">
      <div class="font-bold">{{ log.title }}</div>
      <div>{{ log.message }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Log } from "../../types/types";

const props = defineProps<{
  logs: Log[]
}>();
</script>
