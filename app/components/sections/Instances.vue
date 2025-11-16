<script setup>
import cardTypes from '~/configs/cardTypes';
import sectionFlow from '~/configs/sectionFlow';

const storeInstances = useInstancesStore();
const {
  data: items,
  pending,
  refresh,
  error,
} = useAsyncData(() => storeInstances.fetchInstances());

const contentFlow = ref(sectionFlow.grid);
</script>

<template>
  <div class="sections-dynamic-cards">
    <UiDynamicSectionHeader
      v-model="contentFlow"
      :pending="pending"
      @fetch="refresh"
    />

    <UiError v-if="error" :error="error" />

    <div
      v-auto-animate
      class="sections-dynamic-cards__content"
      :class="{
        [`sections-dynamic-cards__content--${contentFlow}`]: !!contentFlow,
      }"
    >
      <template v-if="!pending || storeInstances.instances?.length">
        <slot
          v-for="item in items || storeInstances.instances"
          :key="item.id"
          name="card"
          :data="item"
        >
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
  display: flex;
  flex-direction: column;
  gap: em(20);

  &__content {
    display: grid;
    gap: em(20);

    &--grid {
      grid-template-columns: repeat(6, 1fr);

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
}
</style>
