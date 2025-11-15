<script setup>
defineProps({
  error: {
    type: Error,
    required: true,
  },
});
</script>

<template>
  <AFade>
    <div v-if="error" class="error">
      <p class="h2-m-r">{{ error.statusCode }}</p>
      <p class="h4-r-r">{{ error.message }}</p>
    </div>
  </AFade>
</template>

<style lang="scss" scoped>
.error {
  display: flex;
  flex-direction: column;
  gap: em(10);
  align-items: center;
  justify-content: center;
  text-align: center;
}
</style>
