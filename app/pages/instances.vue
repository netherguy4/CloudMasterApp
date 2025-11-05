<script setup>
const instancesStore = useInstancesStore();

const { data, error, status, code, pending } = useAsyncData(
  'instances',
  async () => await useRequest('instances_list'),
);
</script>

<template>
  <div class="pages-instances">
    <SectionsDynamicCards
      class="pages-instances__section"
      :items="
        data?.map(({ zone, ...inst }) => ({
          ...inst,
          zone: zone.split('/')[1],
        }))
      "
      :loading="pending"
    />
  </div>
</template>

<style lang="scss" scoped></style>
