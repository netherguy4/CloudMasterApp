<template>
  <div class="layouts-default">
    <main class="layouts-default__content">
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
  }

  &__wrapper {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
  }
}
</style>
