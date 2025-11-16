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
    class="dynamic-cards"
    :items="items || storeInstances.instances"
    :pending="pending"
    :error="error"
    @refresh="refresh"
  >
    <CardInstance class="dynamic-card" v-bind="data" />
  </UiDynamicCards>
</template>
