<script setup>
const instancesStore = useInstancesStore();

const { data, error, status, code, pending } = useAsyncData(
  'instances',
  async () => await useRequest('instances_list'),
);
</script>

<template>
  <div class="pages-instances">
    {{ { data, error, status, code, pending } }}
    <!-- <SectionsDynamicCards
      class="pages-instances__section"
      :items="instancesStore.instances"
      :loading="pending"
    /> -->
  </div>
</template>

<style lang="scss" scoped></style>
