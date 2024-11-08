<template>
  <button @click="handleClick"
    class="px-4 py-2 text-white bg-blue-600 rounded-md transition duration-300 shadow-md hover:bg-blue-700 hover:shadow-lg focus:outline-none focus:ring-4 focus:ring-blue-300">
    <slot></slot>
  </button>
</template>

<script setup lang="ts">
defineProps<{
  onClick?: () => void;
}>();

const emit = defineEmits<{
  (e: 'click'): void;
}>();

const handleClick = () => {
  emit('click');
};
</script>
