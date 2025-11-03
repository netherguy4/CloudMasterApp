<script setup>
const isLayoutCompact = ref(true);

const toggleCompactState = () =>
  isLayoutCompact.value
    ? (isLayoutCompact.value = false)
    : (isLayoutCompact.value = true);
</script>

<template>
  <aside
    class="layout-aside"
    :class="{ ['layout-aside--compact']: isLayoutCompact }"
  >
    <div class="layout-aside__content">
      <CImage class="layout-aside__logo" src="/icons/128x128@2x.png" />

      <LAsideNavigation
        :is-compact="isLayoutCompact"
        class="layout-aside__nav"
      />

      <button
        class="layout-aside__button hidden-desktop"
        @click="toggleCompactState"
      >
        <span v-if="!isLayoutCompact" class="c2-r-r">Hide</span>

        <CIcon class="layout-aside__icon" name="navigation-left" />
      </button>
    </div>
  </aside>
</template>

<style lang="scss" scoped>
.layout-aside {
  $parent: &;

  overflow: auto;
  background-color: $background-color-secondary;
  border-right: 1px solid $border-color-primary;

  &__icon {
    height: em(30);
    transition: $time-normal $ease;
    rotate: 0deg;
  }

  &__content {
    display: flex;
    flex-direction: column;
    gap: em(30);
    min-height: 100%;
    padding-block: em(10) 0;
  }

  &__logo {
    align-self: center;
    width: em(100);
    transition: width $time-normal $ease;

    @include media-breakpoint-down(sm) {
      width: em(70);
    }
  }

  &__button {
    display: flex;
    gap: em(5);
    align-items: center;
    justify-content: center;
    padding: em(5);
    background-color: $actions-color-primary;
    transition: background-color $time-normal $ease-out;

    @include hover-active-focus {
      background-color: $actions-color-secondary;
    }

    @include active {
      background-color: $actions-color-tertiary;
    }
  }

  &__nav {
    flex: 1;
  }

  &--compact {
    @include media-breakpoint-down(sm) {
      #{$parent} {
        &__icon {
          height: em(20);
          rotate: 180deg;
        }

        &__logo {
          width: em(30);
        }
      }
    }
  }
}
</style>
