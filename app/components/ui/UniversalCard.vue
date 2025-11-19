<template>
  <UiGlowWrapper
    class="ui-universal-card"
    @mouseover="ip?.onHoverIn?.()"
    @mouseleave="ip?.onHoverOut?.()"
    @click="ip?.onClick?.()"
  >
    <div class="ui-universal-card__wrapper">
      <div class="ui-universal-card__inner">
        <template v-for="({ label, value }, index) in parts" :key="index">
          <UiSkeletonBlock v-if="pending" />

          <div v-else class="ui-universal-card__part">
            <p class="i2-r">{{ label }}</p>
            <p class="h6-r">{{ value }}</p>
          </div>
        </template>

        <template v-if="button?.text">
          <UiSkeletonBlock
            v-if="pending"
            class="ui-universal-card__button--skeleton"
          />

          <button
            v-else
            class="ui-universal-card__button"
            :class="{
              [`ui-universal-card__button--${button.text.toLowerCase()}`]: true,
            }"
            @click.stop="button.click"
          >
            <span class="i1-r">{{ button.text }}</span>
          </button>
        </template>

        <AFade>
          <UiStatus
            v-if="!pending"
            :status="status"
            class="ui-universal-card__status"
          />
        </AFade>
      </div>
    </div>
  </UiGlowWrapper>
</template>

<script setup>
defineProps({
  parts: {
    type: Array,
    default: () => [],
  },
  ip: {
    type: Object,
    default: () => ({}),
  },
  button: {
    type: Object,
    default: () => ({}),
  },
  status: {
    type: String,
    default: '',
  },
  pending: {
    type: Boolean,
    default: false,
  },
});
</script>

<style lang="scss" scoped>
.ui-universal-card {
  $parent: &;

  display: flex;
  flex-direction: column;
  padding: 2px;
  background-color: $background-color-secondary;
  border-radius: em(8);
  transition: translate $time-normal $ease;

  @include hover {
    translate: 0 em(-3);
  }

  &__wrapper {
    display: inherit;
    flex: 1;
    flex-direction: inherit;
    padding: em(20);
    border-radius: inherit;
  }

  &__inner {
    position: relative;
    display: inherit;
    flex: 1;
    flex-direction: inherit;
    gap: em(10);
    pointer-events: none;
  }

  &__button {
    padding: em(13);
    margin-top: em(10);
    text-align: center;
    text-transform: uppercase;
    pointer-events: none;
    background-color: $accent-color-secondary;
    border-radius: em(8);
    opacity: 0.4;
    transition:
      background-color $time-normal $ease,
      opacity $time-normal $ease;

    &--start {
      pointer-events: auto;
      opacity: 1;

      @include hover-active-focus {
        background-color: $accent-color-success;
      }
    }

    &--stop {
      pointer-events: auto;
      opacity: 1;

      @include hover-active-focus {
        background-color: $accent-color-error;
      }
    }

    &--skeleton {
      min-height: em(49);
      margin-top: em(10);
    }
  }

  &__part {
    @include text-clip(2);

    display: flex;
    flex: 1;
    flex-direction: column;
    user-select: none;
    transition: opacity $time-normal $ease-in-out-sine;

    &:not(&--ip) {
      z-index: 1;
    }
  }

  &__status {
    position: absolute;
    top: 0;
    right: 0;
    z-index: 1;
  }
}
</style>
