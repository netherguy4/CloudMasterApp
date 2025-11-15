<script setup>
import cardTypes from '~/configs/cardTypes';

const storeInstances = useInstancesStore();
const {
  data: items,
  pending,
  error,
} = useAsyncData(() => storeInstances.fetchInstances());
</script>

<template>
  <div class="sections-dynamic-cards">
    <UiError v-if="error" :error="error" />

    <div v-auto-animate class="sections-dynamic-cards__content">
      <template v-if="!pending">
        <slot v-for="item in items" :key="item.id" name="card" :data="item">
          <CardUniversal
            v-if="!$slots.card"
            :card-type="cardTypes.instance"
            v-bind="item"
          />
        </slot>
      </template>

      <template v-else>
        <slot v-for="n in 8" :key="n" name="skeleton">
          <CardUniversalSkeleton
            v-if="!$slots.skeleton"
            :card-type="cardTypes.instance"
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
