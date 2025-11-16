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
    title="Servers"
    :items="items || storeServers.servers"
    :pending="pending"
    :error="error"
    @refresh="refresh"
  >
    <CardServer v-bind="data" :pending="pending" />
  </UiDynamicCards>
</template>
