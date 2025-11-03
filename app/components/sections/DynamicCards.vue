<script setup>
import cardTypes from '~/configs/cardTypes';

cardTypes;
defineProps({
  items: {
    type: Array,
    default: undefined,
  },
  cardType: {
    type: String,
    default: cardTypes.instance,
    validator: (type) => [cardTypes.instance, cardTypes.server].includes(type),
  },
});
</script>

<template>
  <div v-if="items?.length" class="sections-dynamic-cards">
    <div class="sections-dynamic-cards__content">
      <slot v-for="item in items" :key="item.id" name="card" :data="item">
        <CardUniversal
          v-if="!$slots.card"
          :card-type="cardType"
          v-bind="item"
        />
      </slot>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.sections-dynamic-cards {
  &__content {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: em(20);

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
