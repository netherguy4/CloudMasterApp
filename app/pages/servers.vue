<script setup>
const serversStore = useServersStore();

onMounted(serversStore.fetchServers);
</script>

<template>
  <div class="pages-server">
    <SectionsServers
      class="pages-server__section"
      :items="serversStore.servers"
    />
  </div>
</template>

<style lang="scss" scoped></style>
