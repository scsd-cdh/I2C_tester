<template>
  <div class="command-container">
    <select v-model="selectedCommandCode" name="command" id="command-select" class="command-select">
      <option v-for="(command, index) in commands" :key="index" :value="command.code">
        {{ command.title }}
      </option>
    </select>
    <CommandButton @click="runCommand">Run</CommandButton>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { Command } from '../../types/types';
import CommandButton from '../Buttons/EmitButton.vue'; // Adjust path as needed

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

<style scoped>
.command-container {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: center;
  gap: 1rem;
  /* Center vertically */
  text-align: center;
}

.command-select {
  padding: 0.5rem 1rem;
  /* FIXME 'g' char's bottom part gets cut*/
  font-size: 1rem;
  border: 1px solid #ccc;
  border-radius: 4px;
  color: #333;
  max-width: 50vw;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  transition: border-color 0.3s, box-shadow 0.3s;
}

.command-select:focus {
  border-color: #007BFF;
  box-shadow: 0 0 0 3px rgba(0, 123, 255, 0.25);
  outline: none;
}

/* FIXME fix styling (isnt applying to options) */
.command-select option {
  background-color: #fff;
  color: #333;
  font-size: 1rem;
}

.command-select option:hover {
  background-color: #007BFF;
  color: #fff;
}
</style>
