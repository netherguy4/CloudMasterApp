<script setup>
const serversStore = useServersStore();

onMounted(serversStore.fetchServers);
</script>

<template>
  <div class="pages-server">
    <SectionsDynamicCards
      class="pages-server__section"
      :items="serversStore.servers"
      card-type="server"
    />
  </div>
</template>

<style lang="scss" scoped></style>
