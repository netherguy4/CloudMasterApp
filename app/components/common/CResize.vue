<template>
  <div class="resize">
    <slot />
  </div>
</template>

<script setup>
// const { $event } = useNuxtApp();

// onMounted(() => {
//   window.addEventListener('resize', () => {
//     $event('window:resize');
//   });
// });
</script>

<style lang="scss" scoped>
$design-width: 1600;
$design-height: 967;
$default-size: 16;
$minWidth: 768px;
$minHeight: 600px;

// .resize {
//   //font-size: min(
//   //  calc($default-size * max($minWidth, 100vw) / $design-width),
//   //  calc($default-size * max($minHeight, 100vh) / $design-height)
//   //);
//   font-size: min(calc($default-size * max($minWidth, 100vw) / $design-width));

//   @include media-breakpoint-up(xl) {
//     font-size: 16px;
//   }

//   @include media-breakpoint-down(sm) {
//     font-size: $default-size + px;
//   }
// }

.resize {
  // font-size: min(
  //  calc($default-size * max($minWidth, 100vw) / $design-width),
  //  calc($default-size * max($minHeight, 100vh) / $design-height)
  // );

  font-size: min(calc($default-size * max($minWidth, 100vw) / $design-width));

  @include media-breakpoint-up(xl) {
    font-size: $default-size + px;
  }

  @include media-breakpoint-down(md) {
    font-size: min(calc($default-size * max($minWidth, 100vw) / 768));
  }

  @include media-breakpoint-down(sm) {
    font-size: $default-size + px;
  }
}
</style>
