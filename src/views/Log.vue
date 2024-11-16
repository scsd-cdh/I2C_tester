<template>
  <div class="space-y-8">
    <h1 class="page-title text-2xl font-bold text-center text-primary">Run Commands</h1>

    <!-- Toggle Button -->
    <div class="flex justify-center">
      <button @click="toggleOption"
        class="px-4 py-2 bg-primary text-white font-bold rounded-md hover:bg-secondary focus:outline-none focus:ring-2 focus:ring-secondary">
        {{ isDropdownVisible ? 'Switch to Command Input' : 'Switch to Command Dropdown' }}
      </button>
    </div>

    <!-- Conditional Rendering -->
    <div v-if="isDropdownVisible" class="center-container">
      <CommandRun :commands="commands" @run="handleRun"></CommandRun>
    </div>
    <div v-else class="flex justify-center">
      <div class="w-full max-w-md">
        <CommandInput @run="handleRun" />
      </div>
    </div>

    <!-- ConsoleLog Section -->
    <div>
      <ConsoleLog :logs="logs" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import CommandRun from '@c/DropDown/CommandRun.vue';
import ConsoleLog from '@c/Tables/ConsoleLog.vue';
import CommandInput from '@c/TextInput/CommandInput.vue';

import { Command, Log } from '../types/types';
import { invoke } from '@tauri-apps/api/tauri';

// State to toggle between dropdown and input
const isDropdownVisible = ref(true);

const toggleOption = () => {
  isDropdownVisible.value = !isDropdownVisible.value;
};

const logs = ref<Log[]>([]);

async function handleRun(command: Command) {
  let receive = await invoke<string[]>('write_to_pico', { command: command.code });
  const out = receive.join(', ');
  const newLog: Log = {
    title: command.code,
    message: out,
    success: Math.random() < 0.5, // TODO: Update logic for success validation
  };
  logs.value.push(newLog);
}

const commands: Command[] = [
  { title: 'Home Command', desc: 'Takes you to the home page.', code: 'goHome()' },
  { title: 'Command List', desc: 'Displays available commands.', code: 'showCommands()' },
  { title: 'Log Data', desc: 'Logs data to the console.', code: 'logData(data)' },
  { title: 'Settings', desc: 'Opens the settings panel.', code: 'openSettings()' },
];
</script>

<style scoped>
.center-container {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  height: 30vh;
  text-align: center;
}
</style>
