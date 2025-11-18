<script setup>
import sectionFlow from '~/configs/sectionFlow';

const props = defineProps({
  title: {
    type: String,
    default: '',
  },
  pending: {
    type: Boolean,
    default: false,
  },
});

const emit = defineEmits(['fetch']);

const model = defineModel({
  type: String,
  default: sectionFlow.grid,
  validator: (value) => [sectionFlow.grid, sectionFlow.column].includes(value),
});

const changeModel = () => {
  switch (model.value) {
    case sectionFlow.grid: {
      model.value = sectionFlow.column;
      break;
    }
    default: {
      model.value = sectionFlow.grid;
    }
  }
};
</script>

<template>
  <div class="dynamic-section-header">
    <div v-if="title" class="dynamic-section-header__title">
      <p class="h2-r">{{ title }}</p>
    </div>

    <div class="dynamic-section-header__buttons">
      <button class="dynamic-section-header__button">
        <CIcon
          class="dynamic-section-header__icon"
          name="reload"
          :class="{ ['dynamic-section-header__icon--loading']: props.pending }"
          @click="emit('fetch')"
        />
      </button>

      <button class="dynamic-section-header__button" @click="changeModel">
        <CIcon class="dynamic-section-header__icon" :name="model" />
      </button>
    </div>
  </div>
</template>

<style lang="scss" scoped>
@keyframes loading {
  0% {
    rotate: 0;
  }

  100% {
    rotate: 360deg;
  }
}

.dynamic-section-header {
  display: flex;
  gap: em(8);
  align-items: center;

  &__icon {
    width: em(24);
    height: em(24);

    &--loading {
      animation: loading 1.5s linear infinite;
    }
  }

  &__buttons {
    display: flex;
    gap: em(8);
    margin-left: auto;
  }
}
</style>
