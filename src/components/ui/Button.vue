<script setup lang="ts">
defineProps<{
  text: string;
}>();
</script>

<template>
  <button
    class="px-4 py-2 text-white bg-indigo-500 rounded-lg hover:bg-indigo-600 focus:outline-none focus:ring-2 focus:ring-indigo-500">
    {{ text }}
  </button>
</template>
