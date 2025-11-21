<script setup>
import { computed, useTemplateRef } from 'vue';
import { useMouseInElement } from '@vueuse/core';

const props = defineProps({
  size: {
    type: Number,
    default: 150,
  },
  tag: {
    type: String,
    default: 'span',
  },
  glowColor: {
    type: String,
    default: '#ffffff',
  },
  hoverEffect: {
    type: Boolean,
    default: false,
  },
  dimm: {
    type: Boolean,
    default: true,
  },
});

const container = useTemplateRef('container');
const { elementX, elementY } = useMouseInElement(container);

const glowStyle = computed(() => ({
  '--x': `${elementX.value}px`,
  '--y': `${elementY.value}px`,
  '--glow-color': props.glowColor,
  '--glow-size': `${props.size}px`,
}));
</script>

<template>
  <component
    :is="tag"
    ref="container"
    class="glow"
    :class="{
      ['glow--dimm']: dimm,
      ['glow--hover']: hoverEffect,
    }"
    :style="glowStyle"
  >
    <div class="glow__layer glow__base">
      <slot />
    </div>

    <div class="glow__layer glow__overlay" aria-hidden="true">
      <slot />
    </div>
  </component>
</template>

<style lang="scss" scoped>
.glow {
  $parent: &;

  position: relative;
  display: inline-grid;
  vertical-align: middle;

  &__layer {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    transition: opacity $time-normal $ease;
  }

  &__base {
    z-index: 1;
    color: inherit;
    transition: opacity $time-normal $ease;
  }

  &__overlay {
    z-index: 2;
    color: var(--glow-color);
    pointer-events: none;
    user-select: none;
    opacity: 0.7;
    mask-image: radial-gradient(
      calc(var(--glow-size) / 16 * 1em) circle at var(--x) var(--y),
      rgb(255 255 255 / 100%) 0%,
      transparent 100%
    );
    mask-repeat: no-repeat;
    will-change: mask-image;
  }

  > * {
    grid-area: 1 / 1;
  }

  &--dimm {
    #{$parent}__base {
      opacity: 0.7;
    }
  }

  &--hover {
    @include hover {
      #{$parent}__overlay {
        opacity: 1;
      }
    }
    #{$parent}__overlay {
      transition: opacity $time-normal $ease;
    }
  }
}
</style>
