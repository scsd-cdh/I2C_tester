<template>
  <div>
    <h1 class="page-title">List of Available Commands</h1>
    <div class="table-container">
      <Table :commands="commands"></Table>
    </div>
  </div>
</template>

<script setup lang="ts">
import Table from '@c/Tables/RefSheetTable.vue';
import { Command } from '../types/types';

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
.table-container {
  padding: 0 5rem;
  /* Adds padding on the left and right */
}
</style>
