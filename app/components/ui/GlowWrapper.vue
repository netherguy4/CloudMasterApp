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

const container = ref(null);
const glow = ref(null);

useDocumentMouseMove((e) => {
  if (!container.value || !glow.value) return;

  const rect = container.value.getBoundingClientRect();
  const x = e.clientX - rect.left;
  const y = e.clientY - rect.top;

  glow.value.style.setProperty('--x', `${x}px`);
  glow.value.style.setProperty('--y', `${y}px`);
});

const size = computed(() => props.sizeEm + 'em');
</script>

<template>
  <component :is="tag" ref="container" class="glow-container">
    <div ref="glow" class="glow-border"></div>
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
    linear-gradient(rgba($color-white, 0.99) 0 0) content-box,
    linear-gradient(rgba($color-white, 0.99) 0 0);
  mask-composite: xor;
  mask-composite: exclude;
  transition: opacity $time-slow ease;
}
</style>
