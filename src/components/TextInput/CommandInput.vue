<template>
  <div class="p-6 bg-light-background text-light-foreground rounded-lg shadow-md max-w-md border-r-light-foreground ">
    <h2 class="text-xl font-bold mb-6 text-primary text-center">Add a Command</h2>

    <div class="space-y-4">
      <div>
        <label for="title" class="block text-sm font-medium mb-2">Title</label>
        <input id="title" type="text" v-model="command.title" placeholder="Enter command title"
          class="w-full px-4 py-2 border border-light-foreground rounded-md focus:outline-none focus:ring-2 focus:ring-primary" />
      </div>

      <div>
        <label for="desc" class="block text-sm font-medium mb-2">Description</label>
        <input id="desc" v-model="command.desc" rows="1" placeholder="Enter command description"
          class="w-full px-4 py-2 border border-light-foreground rounded-md focus:outline-none focus:ring-2 focus:ring-primary"></input>
      </div>

      <div>
        <label for="code" class="block text-sm font-medium mb-2">Code</label>
        <input id="code" v-model="command.code" rows="1" placeholder="Enter command code"
          class="w-full px-4 py-2 border gray-300 border-light-foreground rounded-md focus:outline-none focus:ring-2 focus:ring-primary font-mono"></input>
      </div>

      <!-- Run Button -->
      <button
        class="w-full py-2 bg-primary text-white font-bold rounded-md hover:bg-secondary focus:outline-none focus:ring-2 focus:ring-secondary"
        @click="handleRun">
        Run Command
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { Command } from '@/types/types';

const command = ref<Command>({
  title: '',
  desc: '',
  code: ''
});

const emit = defineEmits<{
  (e: 'run', command: Command): void;
}>();

const handleRun = () => {
  if (command.value.title && command.value.code) {
    emit('run', command.value);
    // Reset the form after emitting
    command.value = { title: '', desc: '', code: '' };
  }
};
</script>
