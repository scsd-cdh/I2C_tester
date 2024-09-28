<script setup lang="ts">
import { Section } from './types/types';
import UINavbar from '@c/Navbar/UINavbar.vue'

const sections: Section[] = [
  { name: 'Reference Sheet', link: '/ref-sheet', icon: 'src/assets/MenuIcons/ref-sheet.svg' },
  { name: 'Command', link: '/communications', icon: 'src/assets/MenuIcons/command.svg' },
  { name: 'Settings', link: '/settings', icon: 'src/assets/MenuIcons/settings.svg' }
];
</script>

<template>
  <UINavbar :sections="sections"></UINavbar>
  <div class="ml-24">
    <RouterView />
  </div>
</template>
