<script setup>
defineProps({
  items: {
    type: Array,
    default: undefined,
  },
});
</script>

<template>
  <div v-if="items?.length" class="sections-instances">
    <div class="sections-instances__content">
      <SectionsInstancesCard
        v-for="item in items"
        :key="item.id"
        v-bind="item"
        class="sections-instances__card"
      />
    </div>
  </div>
</template>

<style lang="scss" scoped>
.sections-instances {
  &__content {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(em(300), 1fr));
    gap: em(20);
  }
}
</style>
