<script setup>
import sectionFlow from '~/configs/sectionFlow';

const props = defineProps({
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
  <div class="section-header">
    <button class="section-header__button">
      <CIcon
        class="section-header__icon"
        name="reload"
        :class="{ ['section-header__icon--loading']: props.pending }"
        @click="emit('fetch')"
      />
    </button>

    <button class="section-header__button" @click="changeModel">
      <CIcon class="section-header__icon" :name="model" />
    </button>
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

.section-header {
  display: flex;
  gap: em(8);
  justify-content: flex-end;

  &__icon {
    width: em(24);
    height: em(24);

    &--loading {
      animation: loading 1.5s linear infinite;
    }
  }

  &__button {
  }
}
</style>
