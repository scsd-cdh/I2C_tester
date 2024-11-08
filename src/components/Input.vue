<script setup lang="ts">
import Button from '@c/ui/Button.vue'

let inputValue: string = '';

defineEmits<{
  (e: 'getInput', input: string): void;
}>();

</script>

<template>
  <div class="flex flex-col items-center justify-center p-4 bg-gray-800 rounded-lg shadow-md max-w-sm mx-auto">
    <label for="userInput" class="mb-2 font-semibold text-white">Enter command:</label>
    <input v-model="inputValue" type="text" id="userInput"
      class="px-4 py-2 mb-4 bg-gray-700 text-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
      placeholder="Type here..." />
    <Button @click="$emit('getInput', inputValue)" text="Send" />
  </div>
</template>
