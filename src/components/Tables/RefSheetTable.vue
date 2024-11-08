<template>
  <div>
    <div class="max-h-[80vh] overflow-y-scroll border border-gray-300 rounded-md">
      <table class="w-full border-collapse">
        <thead>
          <tr>
            <th class="bg-gray-200 text-left p-2 border-b border-gray-300">Command</th>
            <th class="bg-gray-200 text-left p-2 border-b border-gray-300">Code</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(command, index) in commands" :key="index" class="even:bg-gray-50">
            <td class="p-3 border-b border-gray-200">
              <div class="font-semibold">{{ command.title }}</div>
              <div class="text-sm text-gray-600">{{ command.desc }}</div>
            </td>
            <td class="p-3 border-b border-gray-200">
              <code class="bg-gray-100 p-1 rounded-md">{{ command.code }}</code>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Command } from '../../types/types';

const props = defineProps<{
  commands: Command[];
}>();
</script>
