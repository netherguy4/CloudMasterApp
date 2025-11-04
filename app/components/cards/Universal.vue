<script setup>
import { writeText } from '@tauri-apps/plugin-clipboard-manager'
import cardTypes from '~/configs/cardTypes'

const instancesStore = useInstancesStore()

const props = defineProps({
  cardType: {
    type: String,
    default: cardTypes.instance,
    validator: type => [
      cardTypes.instance,
      cardTypes.server,
    ].includes(type),
  },
  id: {
    type: String,
    default: '',
  },
  name: {
    type: String,
    default: '',
  },
  status: {
    type: String,
    required: true,
  },
  port: {
    type: String,
    default: '',
  },
  preset: {
    type: String,
    default: '',
  },
  external_ips: {
    type: Array,
    default: undefined,
  },
  zone: {
    type: String,
    default: '',
  },
  vm: {
    type: Object,
    default: undefined,
  },
})

const mouseOver = ref(false)
const showMessage = ref(false)
const showError = ref(false)

const handleMouseOver = () => (mouseOver.value = true)
const handleMouseLeave = () => (mouseOver.value = false)

const buttonComputed = computed(() => {
  const status = props.status.toLowerCase()
  let text, click
  switch (status) {
    case 'running': {
      text = 'Stop'
      break
    }
    case 'terminated': {
      text = 'Start'
      break
    }
    default: {
      text = 'Busy'
      break
    }
  }
  switch (props.cardType) {
    case cardTypes.instance: {
      click = () => handleInstanceTrigger()
      break
    }
    case cardTypes.server: {
      break
    }
  }
  return {
    text,
    click,
  }
})

const IpComputed = computed(() => {
  const label = mouseOver.value ? 'Copy IP:' : 'IP:'
  let trueValue
  switch (props.cardType) {
    case cardTypes.instance: {
      trueValue = props.external_ips?.[0]
      break
    }
    case cardTypes.server: {
      trueValue = props.vm?.external_ips?.[0] + ':' + props.port
      break
    }
  }
  const value = showError.value
    ? 'ERROR'
    : showMessage.value
      ? 'COPIED'
      : trueValue

  return {
    label,
    value,
    trueValue,
  }
})

const partsComputed = computed(() => {
  switch (props.cardType) {
    case cardTypes.instance: {
      return [
        {
          type: 'name',
          label: 'Name:',
          value: props.name,
        },
        IpComputed.value,
        {
          type: 'zone',
          label: 'Zone',
          value: props.zone,
        },
      ]
    }
    case cardTypes.server: {
      return [
        {
          type: 'name',
          label: 'Name:',
          value: props.name,
        },
        IpComputed.value,
        {
          type: 'preset',
          label: 'Preset:',
          value: props.preset,
        },
        {
          type: 'vm-name',
          label: 'Instance Name:',
          value: props.vm?.name,
        },
      ]
    }
    default: {
      return []
    }
  }
})

const onClickCard = async (text) => {
  try {
    await writeText(text);
    ((showMessage.value = true),
    setTimeout(
      () => (showMessage.value = false),
      1000,
    ))
  }
  catch (e) {
    console.log(e);
    ((showError.value = true),
    setTimeout(
      () => (showError.value = false),
      1000,
    ))
  }
}

const handleInstanceTrigger = async () => {
  const status = props.status.toLowerCase()

  switch (status) {
    case 'running': {
      await instancesStore.stopInstance(
        props.id,
        props.zone,
      )
      await new Promise(resolve => setTimeout(
        resolve,
        1000,
      ))
      await instancesStore.fetchInstances()
      break
    }
    case 'terminated': {
      await instancesStore.startInstance(
        props.id,
        props.zone,
      )
      await new Promise(resolve => setTimeout(
        resolve,
        1000,
      ))
      await instancesStore.fetchInstances()
      break
    }
    default: {
      break
    }
  }
}
</script>

<template>
  <div
    class="cards-universal"
    @click="onClickCard(IpComputed.trueValue)"
    @mouseover="handleMouseOver"
    @mouseleave="handleMouseLeave"
  >
    <div class="cards-universal__wrapper">
      <div
        v-for="({ label, value }, index) in partsComputed"
        :key="index"
        class="cards-universal__part"
      >
        <p class="i2-r-r">
          {{ $tp(label) }}
        </p>

        <p class="h6-r-r">
          {{ $tp(value) }}
        </p>
      </div>

      <button
        v-if="buttonComputed?.text"
        class="cards-universal__button"
        :class="{
          [`cards-universal__button--${buttonComputed.text.toLowerCase()}`]:
            !!buttonComputed.text,
        }"
        @click.stop="buttonComputed.click"
      >
        <span class="i1-r-r">{{ buttonComputed.text }}</span>
      </button>

      <UiStatus
        class="cards-universal__status"
        :status="status"
      />
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

.cards-universal {
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
    --gradient1: #{$background-color-primary};
  }

  &__wrapper {
    position: relative;
    display: flex;
    flex-direction: column;
    gap: em(10);
    pointer-events: none;
  }

  &__button {
    padding: em(13);
    margin-top: em(10);
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
