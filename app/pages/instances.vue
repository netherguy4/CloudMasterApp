<script setup>
const instancesStore = useInstancesStore();

const { data: instances, pending } = useAsyncData(
  'instances',
  instancesStore.fetchInstances,
  {
    default: () => ({ data: instancesStore.instances || [] }),
  },
);
</script>

<template>
  <div class="pages-instances">
    {{ instances.error }}
    <SectionsDynamicCards
      class="pages-instances__section"
      :items="instances?.data"
      :loading="pending"
    />
  </div>
</template>

<style lang="scss" scoped></style>
