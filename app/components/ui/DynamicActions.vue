<script setup>
import sectionFlow from '~/configs/sectionFlow';

const props = defineProps({
  loading: {
    type: Boolean,
    default: false,
  },
});

const emit = defineEmits(['fetch']);

const model = defineModel({
  type: String,
  default: sectionFlow.grid,
  validator: (value) => [sectionFlow.grid, sectionFlow.column].includes(value),
});

const shutterStyle = computed(() => {
  if (model.value === sectionFlow.column)
    return {
      inset: '0 auto 0 100%',
      translate: '-100%',
    };
  return {
    inset: '0 auto 0 0',
  };
});
</script>

<template>
  <div class="dynamic-actions">
    <template v-if="typeof loading === 'boolean'">
      <button class="dynamic-actions__button" @click="emit('fetch')">
        <CIcon
          name="reload"
          class="dynamic-actions__icon"
          :class="{ ['dynamic-actions__icon--loading']: props.loading }"
        />
      </button>

      <div class="dynamic-actions__divider" />
    </template>

    <div class="dynamic-actions__switch">
      <button
        class="dynamic-actions__button"
        @click="() => (model = sectionFlow.grid)"
      >
        <CIcon name="grid" class="dynamic-actions__icon" />
      </button>

      <button
        class="dynamic-actions__button"
        @click="() => (model = sectionFlow.column)"
      >
        <CIcon name="column" class="dynamic-actions__icon" />
      </button>

      <div :style="shutterStyle" class="dynamic-actions__shutter" />
    </div>
  </div>
</template>

<style lang="scss" scoped>
@keyframes loading {
  0% {
    rotate: 0;
  }

  100% {
    rotate: 360deg;
  }
}

.dynamic-actions {
  display: flex;
  gap: em(12);
  align-items: center;
  padding: em(4);
  background-color: $background-color-secondary;
  border: 1px solid $border-color-primary;
  border-radius: em(8);

  &__icon {
    position: relative;
    z-index: 1;
    width: em(18);
    height: em(18);
    color: $color-white;

    &--loading {
      animation: loading 1.5s linear infinite;
    }
  }

  &__button {
    padding: em(9);
    border-radius: em(6);
    transition: background-color $time-normal $ease;

    @include hover-active-focus {
      background-color: $background-color-tertiary;
    }
  }

  &__divider {
    width: 0;
    height: em(24);
    border-right: 1px solid $border-color-primary;
  }

  &__switch {
    position: relative;
    display: inherit;
    gap: inherit;
    align-items: inherit;
  }

  &__shutter {
    position: absolute;
    aspect-ratio: 1;
    pointer-events: none;
    background-color: $accent-color-secondary;
    border-radius: em(8);
    transition:
      left $time-normal $ease,
      translate $time-normal $ease;
  }
}
</style>
