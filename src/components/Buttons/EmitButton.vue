<template>
  <button @click="handleClick" class="run-button">
    <slot></slot>
  </button>
</template>

<script setup lang="ts">
import { defineProps, defineEmits } from 'vue';

const props = defineProps<{
  onClick?: () => void;
}>();


const emit = defineEmits<{
  (e: 'click'): void;
}>();

const handleClick = () => {
  emit('click');
};
</script>

<style scoped>
.run-button {
  padding: 0.5rem 1rem;
  font-size: 1rem;
  border: none;
  border-radius: 4px;
  background-color: #007BFF;
  color: #fff;
  cursor: pointer;
  transition: background-color 0.3s, box-shadow 0.3s;
}

.run-button:hover {
  background-color: #0056b3;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.run-button:focus {
  outline: none;
  box-shadow: 0 0 0 3px rgba(0, 123, 255, 0.25);
}
</style>
