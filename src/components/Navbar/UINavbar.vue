<template>
  <nav class="flex flex-col items-center p-0 bg-gray-800 text-white w-20 fixed h-full">
    <div class="text-2xl font-bold mb-4">
      <router-link to="/" class="text-white no-underline">L</router-link>
    </div>
    <div class="flex flex-col w-full">
      <router-link v-for="section in sections" :key="section.name" :to="section.link"
        class="flex flex-col items-center justify-center w-full py-2 text-white no-underline transition-colors duration-300 hover:bg-gray-700 hover:text-gray-300"
        active-class="bg-gray-600 font-bold text-white">
        <img :src="section.icon" alt="" class="w-2/5 mb-2" :title="section.name" />
        <span class="hidden">{{ section.name }}</span>
      </router-link>
    </div>
  </nav>
</template>

<script setup lang="ts">
import { Section } from '../../types/types';

const props = defineProps<{
  sections: Section[]
}>();

const emit = defineEmits(['change', 'delete']);
const { sections } = props;
</script>
