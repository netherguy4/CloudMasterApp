<script setup>
import cardTypes from '~/configs/cardTypes'

const props = defineProps({
  cardType: {
    type: String,
    default: undefined,
    validator: type => [cardTypes.instance, cardTypes.server].includes(type),
  },
})

const partsNumber = computed(() => {
  switch (props.cardType) {
    case cardTypes.instance: {
      return 3
    }
    case cardTypes.server: {
      return 4
    }
    default: {
      return 3
    }
  }
})
</script>

<template>
  <div class="cards-universal-skeleton">
    <div class="cards-universal-skeleton__wrapper">
      <div
        v-for="n in partsNumber"
        :key="n"
        class="cards-universal-skeleton__part"
      />

      <button class="cards-universal-skeleton__button" />
    </div>
  </div>
</template>

<style lang="scss" scoped>
@keyframes pulse {
  0% {
    background-position: 25% 0;
  }

  100% {
    background-position: 75% 0;
  }
}

.cards-universal-skeleton {
  $parent: &;

  position: relative;
  padding: em(20);
  overflow: hidden;
  background: linear-gradient(
    45deg,
    $background-color-secondary,
    $background-color-tertiary
  );
  border: 1px solid $border-color-secondary;
  border-radius: em(8);

  &__wrapper {
    position: relative;
    display: flex;
    flex-direction: column;
    gap: em(10);
    pointer-events: none;
  }

  &__button {
    height: em(50);
    margin-top: em(10);
  }

  &__part {
    height: em(45);
  }

  &__part,
  &__button {
    background: linear-gradient(
      90deg,
      rgb(255 255 255 / 0%) 0%,
      rgb(255 255 255 / 10%) 50%,
      rgb(255 255 255 / 0%) 100%
    );
    background-size: 400% 100%;
    border-radius: em(8);
    animation: pulse 1.5s $ease-in-out infinite alternate;
  }
}
</style>
