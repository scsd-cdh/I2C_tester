<template>
  <div>
    <h1 class="page-title">Run Commands</h1>
    <div class="center-container">
      <CommandRun :commands="commands" @run="handleRun"></CommandRun>
    </div>
    <div>
      <ConsoleLog :logs="logs"></ConsoleLog>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import CommandRun from '@c/DropDown/CommandRun.vue'
import ConsoleLog from '@c/Tables/ConsoleLog.vue'

import { Command, Log } from '../types/types';
import { invoke } from '@tauri-apps/api/tauri'

const logs = ref<Log[]>([]);

async function handleRun(command: Command) {
  //const newLog: Log = { title: command.title, message: "fewf", success: true }
  let receive = await invoke<string[]>('write_to_pico', { command: command.code })
  console.log('returned value: ', receive.join(','))
  const out = receive.join(', ');
  const newLog: Log = {
    title: command.code,
    message: out,
    success: Math.random() < 0.5 //TODO: redo when validate output
  }
  logs.value.push(newLog)
}



const commands: Command[] = [
  {
    title: 'Home Command long name ewfewfwefwefewf ewfw efewf wefw fwe wefwe wefwe fwe fwef wefew ',
    desc: 'This command takes you to the home page. long text wfewfwefwe fweif weoifjw oweijo fjoiwefioj wfweio ewjif wejio',
    code: 'goHome()'
  },
  {
    title: 'Command List',
    desc: 'Displays a list of all available commands.',
    code: 'showCommands()'
  },
  {
    title: 'Log Data',
    desc: 'Logs the data to the console for debugging.',
    code: 'logData(data)'
  },
  {
    title: 'Settings',
    desc: 'Opens the settings panel.',
    code: 'openSettings()',
  },
  {
    title: 'Home Command',
    desc: 'This command takes you to the home page.',
    code: 'goHome()'
  },
  {
    title: 'Command List',
    desc: 'Displays a list of all available commands.',
    code: 'showCommands()'
  },
  {
    title: 'Log Data',
    desc: 'Logs the data to the console for debugging.',
    code: 'logData(data)'
  },
  {
    title: 'Settings',
    desc: 'Opens the settings panel.',
    code: 'openSettings()',
  },
  {
    title: 'Home Command',
    desc: 'This command takes you to the home page.',
    code: 'goHome()'
  },
  {
    title: 'Command List',
    desc: 'Displays a list of all available commands.',
    code: 'showCommands()'
  },
  {
    title: 'Log Data',
    desc: 'Logs the data to the console for debugging.',
    code: 'logData(data)'
  },
  {
    title: 'Settings',
    desc: 'Opens the settings panel.',
    code: 'openSettings()',
  },
  {
    title: 'Home Command',
    desc: 'This command takes you to the home page.',
    code: 'goHome()'
  },
  {
    title: 'Command List',
    desc: 'Displays a list of all available commands.',
    code: 'showCommands()'
  },
  {
    title: 'Log Data',
    desc: 'Logs the data to the console for debugging.',
    code: 'logData(data)'
  },
  {
    title: 'Settings',
    desc: 'Opens the settings panel.',
    code: 'openSettings()',
  },
];

</script>
<style scoped>
.center-container {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  height: 30vh;
  /* Full viewport height */
  text-align: center;
}
</style>
