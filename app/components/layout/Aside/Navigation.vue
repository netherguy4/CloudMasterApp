<script setup>
import navigationRoutes from '~/configs/navigationRoutes';
</script>

<template>
  <nav class="layout-aside-navigation">
    <ul v-if="navigationRoutes.length" class="layout-aside-navigation__list">
      <li
        v-for="({ title, link }, index) in navigationRoutes"
        :key="index"
        class="layout-aside-navigation__li"
      >
        <CTagDetect :to="link" class="layout-aside-navigation__link">
          <p class="h4-r-r">{{ $tp(title) }}</p>
        </CTagDetect>
      </li>
    </ul>
  </nav>
</template>

<style lang="scss" scoped>
.layout-aside-navigation {
  &__link {
    display: block;
    padding: em(10) em(20);
    transition: background-color $time-normal $ease;

    @include hover-active-focus {
      background-color: $actions-color-primary;
    }

    @include media-breakpoint-down(sm) {
      padding: em(5) em(10);
    }

    &.router-link-active {
      background-color: $actions-color-secondary;
    }
  }
}
</style>
