<script setup lang="ts">
import Input from '@c/Input.vue'
import { ref } from 'vue';

// When using the Tauri API npm package:
import { invoke } from '@tauri-apps/api/tauri'

let input = ref('');

function parseData(data: string) {
  input.value = data
  invoke('my_custom_command')
  //TODO: return value of the command
}

</script>

<template>
  <div>
    <Input @get-input="parseData($event)"></Input>
    <h2>Show output: {{ input }}</h2>
  </div>
</template>
