<script setup>
const instancesStore = useInstancesStore();

const { data: instances } = useAsyncData(
  'instances',
  instancesStore.fetchInstances,
  { default: () => ({ data: instancesStore.instances || [] }) },
);
</script>

<template>
  <div class="pages-instances">
    <SectionsDynamicCards
      class="pages-instances__section"
      :items="instances?.data"
      :loading="instancesStore.loading"
    />
  </div>
</template>

<style lang="scss" scoped></style>
