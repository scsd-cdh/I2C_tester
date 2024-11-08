<template>
  <div :class="themeClass">
    <div class="flex flex-col items-center justify-center min-h-screen bg-light-background dark:bg-dark-background">
      <h1 class="text-light-foreground dark:text-dark-foreground text-3xl font-semibold">Theme Switcher</h1>
      <button @click="toggleTheme"
        class="px-6 py-3 mt-4 bg-primary text-white rounded-lg hover:bg-secondary focus:ring-4 focus:ring-indigo-300 dark:bg-secondary dark:hover:bg-primary">
        Toggle Dark Theme
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';

const isDarkMode = ref(false);

// Computed class based on theme state
const themeClass = computed(() => (isDarkMode.value ? 'dark' : ''));

// Toggle dark mode on the `html` element
const toggleTheme = () => {
  isDarkMode.value = !isDarkMode.value;
  if (isDarkMode.value) {
    document.documentElement.classList.add('dark');
  } else {
    document.documentElement.classList.remove('dark');
  }
};
</script>
