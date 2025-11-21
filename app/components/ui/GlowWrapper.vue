<script setup>
const props = defineProps({
  tag: {
    type: String,
    default: 'div',
  },
  sizeEm: {
    type: Number,
    default: 18,
  },
});

const container = useTemplateRef('container');

const { elementX, elementY } = useMouseInElement(container);

const glowStyle = computed(() => ({
  '--x': `${elementX.value}px`,
  '--y': `${elementY.value}px`,
}));

const size = computed(() => props.sizeEm + 'em');
</script>

<template>
  <component :is="tag" ref="container" class="glow-container">
    <div :style="glowStyle" class="glow-border"></div>
    <slot />
  </component>
</template>

<style lang="scss" scoped>
.glow-container {
  position: relative;

  @include hover {
    .glow-border {
      opacity: 1;
    }
  }
}

.glow-border {
  position: absolute;
  inset: 0;
  padding: inherit;
  pointer-events: none;
  background: radial-gradient(
    v-bind(size) circle at var(--x) var(--y),
    rgba($color-white, 0.5),
    $border-color-secondary 70%,
    transparent 100%
  );
  border-radius: inherit;
  opacity: 0.4;
  mask:
    linear-gradient(rgba($color-white, 0.995) 0 0) content-box,
    linear-gradient(rgba($color-white, 0.995) 0 0);
  mask-composite: xor;
  mask-composite: exclude;
  transition: opacity $time-slow ease;
}
</style>
