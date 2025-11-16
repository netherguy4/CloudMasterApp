<script setup>
const storeInstances = useInstancesStore();

const {
  data: items,
  pending,
  refresh,
  error,
} = useAsyncData(() => storeInstances.fetchInstances());
</script>

<template>
  <UiDynamicCards
    v-slot="{ data }"
    title="Instances"
    :items="items || storeInstances.instances"
    :pending="pending"
    :error="error"
    @refresh="refresh"
  >
    <CardInstance v-bind="data" :pending="pending" />
  </UiDynamicCards>
</template>
