<script setup></script>

<template>
  <aside class="layout-aside">
    <div class="layout-aside__content">
      <CImage class="layout-aside__logo" src="/icons/128x128@2x.png" />

      <LAsideNavigation class="layout-aside__nav" />
    </div>
  </aside>
</template>

<style lang="scss" scoped>
.layout-aside {
  display: flex;
  background-color: $background-color-secondary;
  border-right: 1px solid $border-color-primary;

  &__content {
    display: flex;
    flex: 1;
    flex-direction: column;
    gap: em(30);
    padding-block: em(10);
  }

  &__logo {
    align-self: center;
    width: em(100);
  }

  &__nav {
    flex: 1;
  }
}
</style>
