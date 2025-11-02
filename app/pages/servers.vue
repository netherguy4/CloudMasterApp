<script setup>
const instancesStore = useInstancesStore();

onMounted(instancesStore.fetchInstances);
</script>

<template>
  <div class="pages-server">
    <SectionsInstances
      class="pages-server__section"
      :items="instancesStore.instances"
    />
  </div>
</template>

<style lang="scss" scoped></style>
