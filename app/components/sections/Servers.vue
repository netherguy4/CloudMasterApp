<script setup>
const storeServers = useServersStore();

const {
  data: items,
  pending,
  refresh,
  error,
} = useAsyncData(() => storeServers.fetchServers());
</script>

<template>
  <UiDynamicCards
    v-slot="{ data }"
    title="Instances"
    class="dynamic-cards"
    :items="items || storeServers.servers"
    :pending="pending"
    :error="error"
    @refresh="refresh"
  >
    <CardServer class="dynamic-card" v-bind="data" />
  </UiDynamicCards>
</template>
