<script setup>
import { writeText } from '@tauri-apps/plugin-clipboard-manager';

const props = defineProps({
  name: {
    type: String,
    default: '',
  },
  id: {
    type: String,
    required: true,
  },
  status: {
    type: String,
    required: true,
  },
  internal_ips: {
    type: Array,
    default: undefined,
  },
  external_ips: {
    type: Array,
    default: undefined,
  },
  zone: {
    type: String,
    default: '',
  },
});

const showMessage = ref(false);
const showError = ref(false);

const buttonComputed = computed(() => {
  const status = props.status.toLowerCase();
  let text;
  switch (status) {
    case 'running': {
      text = 'Stop';
      break;
    }
    case 'terminated': {
      text = 'Start';
      break;
    }
    default: {
      text = 'Busy';
      break;
    }
  }
  return {
    text,
  };
});

const onClickCard = async (text) => {
  try {
    await writeText(text);
    ((showMessage.value = true),
      setTimeout(() => (showMessage.value = false), 1000));
  } catch (e) {
    console.log(e);
    ((showError.value = true),
      setTimeout(() => (showError.value = false), 1000));
  }
};
</script>

<template>
  <div class="cards-instance">
    <div class="cards-instance__wrapper">
      <div class="cards-instance__part">
        <p class="i2-r-r">Name:</p>

        <p class="h6-r-r">{{ name }}</p>
      </div>

      <div class="cards-instance__part cards-instance__part--ip">
        <p class="i2-r-r">IP:</p>

        <p class="h6-r-r">{{ external_ips[0] }}</p>
      </div>

      <div class="cards-instance__part">
        <p class="i2-r-r">Zone:</p>

        <p class="h6-r-r">{{ zone }}</p>
      </div>

      <UiStatus class="cards-instance__status" :status="status" />
    </div>

    <div class="cards-instance__overlay">
      <div
        v-if="external_ips[0]"
        class="cards-instance__part"
        @click="onClickCard(external_ips[0])"
      >
        <p class="i2-r-r">Copy IP:</p>

        <p v-if="showMessage" class="h6-r-r">COPIED</p>

        <p v-else-if="showError" class="h6-r-r">ERROR</p>

        <p v-else class="h6-r-r">{{ external_ips[0] }}</p>
      </div>

      <button
        v-if="buttonComputed?.text"
        class="cards-instance__button"
        :class="{
          [`cards-instance__button--${buttonComputed.text.toLowerCase()}`]:
            !!buttonComputed.text,
        }"
      >
        <span class="i1-r-r">{{ buttonComputed.text }}</span>
      </button>
    </div>
  </div>
</template>

<style lang="scss" scoped>
@property --gradient1 {
  initial-value: $background-color-secondary;
  inherits: true;
  syntax: '<color>';
}

@property --gradient2 {
  initial-value: $background-color-tertiary;
  inherits: true;
  syntax: '<color>';
}

@property --breakpoint1 {
  initial-value: 0%;
  inherits: true;
  syntax: '<percentage>';
}

@property --breakpoint2 {
  initial-value: 100%;
  inherits: true;
  syntax: '<percentage>';
}

@property --angle {
  initial-value: 45deg;
  inherits: true;
  syntax: '<angle>';
}

.cards-instance {
  $parent: &;

  position: relative;
  padding: em(20);
  overflow: hidden;
  cursor: pointer;
  background: linear-gradient(
    var(--angle),
    var(--gradient1) var(--breakpoint1),
    var(--gradient2) var(--breakpoint2)
  );
  border: 1px solid $border-color-secondary;
  border-radius: em(8);
  transition-timing-function: $ease-out;
  transition-duration: $time-normal;
  transition-property:
    --gradient1, --gradient2, --breakpoint1, --breakpoint2, --angle, box-shadow;

  @include hover-active-focus {
    --breakpoint1: 100%;
    --angle: 90deg;
    #{$parent} {
      &__overlay {
        opacity: 1;
      }

      &__part {
        &--ip {
          opacity: 0;
        }
      }
    }
  }

  &__wrapper {
    position: relative;
    display: flex;
    flex-direction: column;
    gap: em(10);
    pointer-events: none;
  }

  &__overlay {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    padding: inherit;
    border-radius: inherit;
    opacity: 0;
    transition: opacity $time-normal $ease-in-out-sine;
  }

  &__button {
    min-width: em(100);
    padding: em(13);
    text-align: center;
    text-transform: uppercase;
    pointer-events: none;
    background-color: $accent-color-secondary;
    border-radius: em(8);
    opacity: 0.4;
    transition:
      background-color $time-normal $ease,
      opacity $time-normal $ease;

    &--start {
      pointer-events: auto;
      opacity: 1;

      @include hover-active-focus {
        background-color: $accent-color-success;
      }
    }

    &--stop {
      pointer-events: auto;
      opacity: 1;

      @include hover-active-focus {
        background-color: $accent-color-error;
      }
    }
  }

  &__part {
    @include text-clip(2);

    display: flex;
    flex: 1;
    flex-direction: column;
    user-select: none;
    transition: opacity $time-normal $ease-in-out-sine;

    &:not(&--ip) {
      z-index: 1;
    }
  }

  &__status {
    position: absolute;
    top: 0;
    right: 0;
    z-index: 1;
  }
}
</style>
