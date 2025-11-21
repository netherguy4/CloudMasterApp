<script setup>
const props = defineProps({
  items: {
    type: Array,
    required: true,
  },
  labelBy: {
    type: String,
    default: 'title',
  },
  trackBy: {
    type: String,
    default: 'id',
  },
});

const model = defineModel({
  type: Object,
  required: true,
});

const itemList = computed(() =>
  props.items.filter(
    (item) => item[props.trackBy] !== model.value[props.trackBy],
  ),
);

function onChange(item, close) {
  model.value = item;
  close();
}
</script>

<template>
  <CDropdown
    position="bottom-down"
    :gap="-1"
    content-full-width
    class="dropdown-accordion"
  >
    <template #trigger="{ visible }">
      <div
        class="dropdown-accordion__selected"
        :class="{ 'dropdown-accordion__selected--opened': visible }"
      >
        <button class="dropdown-accordion__button">
          <span class="dropdown-accordion__font">
            {{ model[labelBy] }}
          </span>

          <CIcon name="chevron-navigation" class="dropdown-accordion__icon" />
        </button>
      </div>
    </template>

    <template #content="{ close }">
      <div class="dropdown-accordion__wrapper">
        <ul class="dropdown-accordion__list">
          <li
            v-for="item in itemList"
            :key="item[trackBy]"
            class="dropdown-accordion__item"
            :class="{
              'dropdown-accordion__item--selected':
                model[trackBy] === item[trackBy],
            }"
            @click="onChange(item, close)"
          >
            <span class="dropdown-accordion__font">
              {{ item[labelBy] }}
            </span>
          </li>
        </ul>
      </div>
    </template>
  </CDropdown>
</template>

<style lang="scss" scoped>
.dropdown-accordion {
  $parent: &;

  &__font {
    font-size: em(16);
    line-height: 1.4;
  }

  &__selected {
    background-color: $background-color-primary;
    border: solid 1px $border-color-primary;

    #{$parent}__icon {
      width: em(12);
      height: em(12);
      color: $accent-color-primary;
      rotate: 90deg;
      transition: rotate $time-normal $ease-out;
    }

    &--opened {
      #{$parent}__icon {
        rotate: -90deg;
      }
    }
  }

  &__button {
    position: relative;
    z-index: 1;
    display: flex;
    gap: em(10);
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding-block: em(8);
    padding-inline: em(16);
    cursor: pointer;
    user-select: none;
  }

  &__wrapper {
    width: 100%;
    padding-inline: em(16);
    margin-top: em(-2);
    background-color: $background-color-primary;
    border: solid 1px $border-color-primary;
    border-top: none;
  }

  &__list {
    display: flex;
    flex-direction: column;
    gap: em(8);
    padding-bottom: em(8);
  }

  &__item {
    padding: em(5) 0;
    cursor: pointer;
  }
}
</style>
