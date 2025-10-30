<script setup>
import { Command } from '@tauri-apps/plugin-shell';

const command = Command.sidecar('binaries/node-server');
const output = await command.execute();
</script>

<template>
  <div class="page">
    <p class="h2-r-i">{{ output?.stdout }}</p>
  </div>
</template>

<style lang="scss" scoped></style>
