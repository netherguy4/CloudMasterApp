<script setup>
import navigationRoutes from '~/configs/navigationRoutes';

defineProps({
  isCompact: {
    type: Boolean,
    default: true,
  },
});
</script>

<template>
  <nav
    class="layout-aside-navigation"
    :class="{ [`layout-aside-navigation--compact`]: isCompact }"
  >
    <ul v-if="navigationRoutes.length" class="layout-aside-navigation__list">
      <li
        v-for="({ title, link, icon }, index) in navigationRoutes"
        :key="index"
        class="layout-aside-navigation__li"
      >
        <CTagDetect :to="link" class="layout-aside-navigation__link">
          <span class="h4-r-r">{{ $tp(title) }}</span>

          <CIcon
            v-if="icon"
            :name="icon"
            class="layout-aside-navigation__icon"
          />
        </CTagDetect>
      </li>
    </ul>
  </nav>
</template>

<style lang="scss" scoped>
.layout-aside-navigation {
  $parent: &;

  &__icon {
    height: em(60);
    transition: height $time-normal $ease;

    @include media-breakpoint-down(sm) {
      height: em(40);
    }
  }

  &__link {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: em(10) em(20);
    text-align: center;
    transition: $time-normal $ease;

    @include hover-active-focus {
      background-color: $actions-color-primary;
    }

    &.router-link-active {
      background-color: $actions-color-secondary;
    }
  }

  &--compact {
    @include media-breakpoint-down(sm) {
      #{$parent} {
        &__icon {
          height: em(20);
        }

        &__link {
          padding: em(10);

          & > span {
            display: none;
          }
        }
      }
    }
  }
}
</style>
