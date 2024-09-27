<script setup lang="ts">
import Input from '@c/Input.vue'
import { ref } from 'vue';

// When using the Tauri API npm package:
import { invoke } from '@tauri-apps/api/tauri'

let input = ref('');

async function parseData(data: string) {
  let receive = await invoke<string[]>('write_to_pico', { command: data })
  //invoke('my_custom_command', { message: data })
  //let receive = await invoke<string>('play_with_piko')
  input.value = receive.join(', ');
}

</script>

<template>
  <div>
    <Input @get-input="parseData($event)"></Input>
    <h2>Show output: {{ input }}</h2>
  </div>
</template>
