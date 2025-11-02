<script setup>
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import instanceStatus from '~/configs/instanceStatus';

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
    validator: (status) => [...instanceStatus].includes(status.toUpperCase()),
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
  <div class="sections-instances-card">
    <div class="sections-instances-card__wrapper">
      <div class="sections-instances-card__name">
        <p class="i2-r-r">Name:</p>

        <p class="h6-r-r">{{ name }}</p>
      </div>

      <div
        class="sections-instances-card__status"
        :class="{
          [`sections-instances-card__status--${status.toLowerCase()}`]:
            !!status,
        }"
      />
    </div>

    <div class="sections-instances-card__overlay">
      <div
        v-if="internal_ips[0]"
        class="sections-instances-card__ip"
        @click="onClickCard(internal_ips[0])"
      >
        <p class="i2-r-r">Copy IP:</p>

        <p v-if="showMessage" class="h6-r-r">COPIED</p>

        <p v-else-if="showError" class="h6-r-r">ERROR</p>

        <p v-else class="h6-r-r">{{ internal_ips[0] }}</p>
      </div>

      <button
        v-if="buttonComputed?.text"
        class="sections-instances-card__button"
        :class="{
          [`sections-instances-card__button--${buttonComputed.text.toLowerCase()}`]:
            !!buttonComputed.text,
        }"
      >
        <span class="i1-r-r">{{ buttonComputed.text }}</span>
      </button>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.sections-instances-card {
  $parent: &;

  position: relative;
  padding: em(20);
  overflow: hidden;
  cursor: pointer;
  background: linear-gradient(
    45deg,
    $background-color-secondary,
    $background-color-tertiary
  );
  border: 1px solid $border-color-primary;
  border-radius: em(8);

  @include hover-active-focus {
    #{$parent} {
      &__overlay {
        backdrop-filter: blur(em(1000));
        opacity: 1;
      }
    }
  }

  &__wrapper {
    position: relative;
    display: flex;
  }

  &__overlay {
    position: absolute;
    inset: 0;
    display: flex;
    padding: inherit;
    backdrop-filter: blur(em(0));
    border-radius: inherit;
    opacity: 0;
    transition:
      backdrop-filter $time-normal $ease-in-out-sine,
      opacity $time-normal $ease-in-out-sine;
  }

  &__button {
    padding: em(5) em(25);
    background-color: $background-color-secondary;
    border-radius: em(8);
    transition: background-color $time-normal $ease;

    &--start {
      @include hover-active-focus {
        background-color: $accent-color-success;
      }
    }

    &--stop {
      @include hover-active-focus {
        background-color: $accent-color-error;
      }
    }
  }

  &__name,
  &__ip {
    flex: 1;
  }

  &__status {
    position: absolute;
    top: em(-5);
    right: em(-5);

    // z-index: 1;
    width: em(12);
    height: em(12);
    background-color: $accent-color-warning;
    border-radius: 50%;

    &--running {
      background-color: $accent-color-success;
    }

    &--terminated {
      background-color: $accent-color-error;
    }
  }
}
</style>
