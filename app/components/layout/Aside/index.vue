<script setup></script>

<template>
  <UiGlowWrapper tag="aside" :size-em="30" class="layout-aside">
    <div class="layout-aside__content">
      <div class="layout-aside__header">
        <CImage class="layout-aside__logo" src="/icons/logo.svg" />

        <div class="layout-aside__title">
          <p class="h3-s">Cloud Master</p>
        </div>

        <div class="layout-aside__subtitle">
          <p class="h4-r">Game Server Management</p>
        </div>
      </div>

      <LAsideNavigation
        :is-compact="isLayoutCompact"
        class="layout-aside__nav"
      />
    </div>
  </UiGlowWrapper>
</template>

<style lang="scss" scoped>
.layout-aside {
  $parent: &;

  padding-right: 2px;
  background-color: $background-color-secondary;

  &__header {
    display: flex;
    flex-direction: column;
    gap: em(8);
    align-items: center;
    text-align: center;
  }

  &__title {
    color: $text-color-primary;
  }

  &__subtitle {
    color: $text-color-tertiary;
  }

  &__content {
    display: flex;
    flex-direction: column;
    gap: em(32);
    min-width: em(240);
    min-height: 100%;
    padding: em(24) em(16);
  }

  &__logo {
    width: em(100);
  }

  &__nav {
    flex: 1;
  }
}
</style>
