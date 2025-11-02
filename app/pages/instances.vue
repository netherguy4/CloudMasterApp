<script setup>
const instancesStore = useInstancesStore();

onMounted(instancesStore.fetchInstances);
</script>

<template>
  <div class="pages-instances">
    <SectionsInstances
      class="pages-instances__section"
      :items="instancesStore.instances"
    />
  </div>
</template>

<style lang="scss" scoped></style>
