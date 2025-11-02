<template>
  <div class="layouts-default">
    <!-- <LHeader class="layouts-default__header" /> -->

    <main class="layouts-default__content">
      <LAside class="layouts-default__aside" />

      <div class="layouts-default__wrapper">
        <slot class-name="layouts-default__view" />
      </div>
    </main>
  </div>
</template>

<script setup></script>

<style scoped lang="scss">
.layouts-default {
  $parent: &;

  display: flex;
  flex-direction: column;

  &:deep(#{$parent}__view) {
    display: flex;
    flex-grow: 1;
    flex-direction: column;

    // padding-top: em($header-default-size);

    // @include media-breakpoint-down(sm) {
    //   padding-top: em($header-adaptive-size);
    // }
  }

  // &__header {
  //   position: fixed;
  //   inset: 0 0 auto;
  //   height: em($header-default-size);

  //   @include media-breakpoint-down(sm) {
  //     height: em($header-adaptive-size);
  //   }
  // }

  &__content {
    display: flex;
  }

  &__wrapper {
    display: flex;
    flex: 1;
    flex-direction: column;
    min-height: 100vh;
  }
}
</style>
