<template>
  <div class="flex flex-row items-center justify-center gap-4 text-center">
    <select v-model="selectedCommandCode" name="command" id="command-select"
      class="px-4 py-2 text-gray-800 text-base border border-gray-300 rounded-md max-w-[50vw] shadow-sm focus:border-blue-500 focus:ring-4 focus:ring-blue-300 transition">
      <option v-for="(command, index) in commands" :key="index" :value="command.code"
        class="bg-white text-gray-800 text-base hover:bg-blue-600 hover:text-white">
        {{ command.title }}
      </option>
    </select>
    <CommandButton @click="runCommand">Run</CommandButton>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { Command } from '../../types/types';
import CommandButton from '../Buttons/EmitButton.vue';

const props = defineProps<{
  commands: Command[]
}>();

const emit = defineEmits<{
  (e: 'run', command: Command): void
}>();

const { commands } = props;
const selectedCommandCode = ref<string>(commands[0]?.code || '');

// Computed property to find the selected command object
const selectedCommand = computed(() =>
  commands.find(command => command.code === selectedCommandCode.value));

// Method to emit the selected command
const runCommand = () => {
  if (selectedCommand.value) {
    emit('run', selectedCommand.value);
    console.log("run command: ", selectedCommand.value.code);
  }
};
</script>
