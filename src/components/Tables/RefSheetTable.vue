<template>
  <div>
    <div class="table-container">
      <table class="commands-table">
        <thead>
          <tr>
            <th>Command</th>
            <th>Code</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(command, index) in commands" :key="index">
            <td>
              <div class="command-title">{{ command.title }}</div>
              <div class="command-description">{{ command.desc }}</div>
            </td>
            <td>
              <code>{{ command.code }}</code>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Command } from '../../types/types'


const props = defineProps<{
  commands: Command[];
}>();

const { commands } = props;
</script>

<style scoped>
.table-container {
  max-height: 80vh;
  /* Adjust the height as needed */
  overflow-y: scroll;
  border: 1px solid #ddd;
  /* Optional: adds a border around the container */
}

.commands-table {
  width: 100%;
  border-collapse: collapse;
}

.commands-table th,
.commands-table td {
  border: 1px solid #ddd;
  padding: 8px;
}

.commands-table th {
  background-color: #f2f2f2;
  text-align: left;
}

.command-title {
  font-weight: bold;
}

.command-description {
  font-size: 0.9em;
  color: #666;
}

code {
  background-color: #f9f9f9;
  padding: 4px;
  border-radius: 4px;
}
</style>
