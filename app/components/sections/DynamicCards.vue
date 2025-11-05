<script setup>
import cardTypes from '~/configs/cardTypes';

defineProps({
  items: {
    type: Array,
    default: () => [],
  },
  cardType: {
    type: String,
    default: cardTypes.instance,
    validator: (type) => [cardTypes.instance, cardTypes.server].includes(type),
  },
  loading: {
    type: Boolean,
    default: false,
  },
});

const [contentRef, enable] = useAutoAnimate();
onMounted(async () => {
  enable?.(false);
  await nextTick();
  enable?.(true);
});
</script>

<template>
  <div class="sections-dynamic-cards">
    <div ref="contentRef" class="sections-dynamic-cards__content">
      <template v-if="items?.length || !loading">
        <slot v-for="item in items" :key="item.id" name="card" :data="item">
          <CardUniversal
            v-if="!$slots.card"
            :card-type="cardType"
            v-bind="item"
          />
        </slot>
      </template>

      <template v-else>
        <slot v-for="n in 8" :key="n" name="skeleton">
          <CardUniversalSkeleton
            v-if="!$slots.skeleton"
            :card-type="cardType"
          />
        </slot>
      </template>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.sections-dynamic-cards {
  &__content {
    display: grid;
    grid-template-columns: repeat(6, 1fr);
    gap: em(20);

    @include media-breakpoint-down(xxl) {
      grid-template-columns: repeat(4, 1fr);
    }

    @include media-breakpoint-down(lg) {
      grid-template-columns: repeat(3, 1fr);
    }

    @include media-breakpoint-down(md) {
      grid-template-columns: repeat(2, 1fr);
    }

    @include media-breakpoint-down(sm) {
      grid-template-columns: 1fr;
    }
  }
}
</style>
