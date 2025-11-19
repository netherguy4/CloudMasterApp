<script setup>
import navigationRoutes from '~/configs/navigationRoutes';
</script>

<template>
  <nav class="layout-aside-navigation">
    <div
      v-for="({ title: groupTitle, items }, groupIndex) in navigationRoutes"
      :key="groupIndex"
      class="layout-aside-navigation__group"
    >
      <div v-if="groupTitle" class="layout-aside-navigation__title">
        <p class="s3-r">{{ groupTitle }}</p>
      </div>

      <ul v-if="items?.length" class="layout-aside-navigation__list">
        <li
          v-for="({ title, link, icon }, index) in items"
          :key="index"
          class="layout-aside-navigation__li"
        >
          <CTagDetect :to="link" class="layout-aside-navigation__link">
            <CIcon
              v-if="icon"
              :name="icon"
              class="layout-aside-navigation__icon"
            />
            <span class="i2-r">{{ title }}</span>
          </CTagDetect>
        </li>
      </ul>
    </div>
  </nav>
</template>

<style lang="scss" scoped>
.layout-aside-navigation {
  $parent: &;

  display: flex;
  flex-direction: column;
  gap: em(16);

  &__icon {
    width: em(16);
    height: em(16);
  }

  &__title {
    padding-inline: em(12);
    color: $text-color-tertiary;
    text-transform: uppercase;
  }

  &__list {
    display: inherit;
    flex-direction: inherit;
    gap: inherit;
  }

  &__group {
    display: inherit;
    flex-direction: inherit;
    gap: em(8);
  }

  &__link {
    display: flex;
    gap: em(10);
    align-items: center;
    padding: em(10) em(12);
    color: $text-color-secondary;
    border-radius: em(8);
    transition:
      background-color $time-normal $ease,
      color $time-normal $ease;

    @include hover-active-focus {
      color: $color-white;
      background-color: $background-color-tertiary;
    }

    &.router-link-active {
      color: $color-white;
      background-color: $accent-color-secondary;
    }
  }
}
</style>
