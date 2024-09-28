<template>
  <nav class="navbar">
    <div class="navbar-brand">
      <router-link to="/" class="navbar-item">GS</router-link>
    </div>
    <div class="navbar-menu">
      <router-link v-for="section in sections" :key="section.name" :to="section.link" class="navbar-item"
        active-class="active">
        <img :src="`${section.icon}`" alt="" class="navbar-icon" :title="`${section.name}`" />
        <span class="navbar-text">{{ section.name }}</span>
      </router-link>
    </div>
  </nav>
</template>

<script setup lang="ts">
import { Section } from '../../types/types' // Adjust the import path as needed

const props = defineProps<{
  sections: Section[]
}>();

const emit = defineEmits(['change', 'delete']);

// Suppress the unused variable warning
// eslint-disable-next-line no-unused-vars
const { sections } = props;

// Additional setup code can go here
</script>

<style scoped>
.navbar {
  display: flex;
  flex-direction: column;
  justify-content: flex-start;
  align-items: center;
  /* Center items horizontally */
  padding: 0rem;
  background-color: #333;
  color: #fff;
  width: 5rem;
  /* Initial width of the vertical navbar */
  position: fixed;
  height: 100%;
}

.navbar-brand .navbar-item {
  font-size: 1.5rem;
  font-weight: bold;
  color: #fff;
  text-decoration: none;
  margin-bottom: 1rem;
}

.navbar-menu {
  display: flex;
  flex-direction: column;
  gap: 0rem;
  width: 100%;
}

.navbar-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: #fff;
  text-decoration: none;
  font-size: 1rem;
  transition: color 0.3s, background-color 0.3s, width 0.3s;
  overflow: hidden;
  width: 100%;
  padding: 0.5rem 0;
}

.navbar-icon {
  width: 40%;
  margin-bottom: 0.5rem;
}

.navbar-text {
  display: none;
  /* Hide text by default */
}

.navbar-menu {
  .navbar-item:hover {
    background-color: #444;
    color: #ddd;
    /* Expand width on hover */
  }

  .navbar-item.active {
    background-color: #555;
    color: #fff;
    font-weight: bold;
  }
}
</style>
