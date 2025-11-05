<script setup>
import statusConfig from '~/configs/statusConfig';

const props = defineProps({
  status: {
    type: String,
    required: true,
    validator: (status) => [...statusConfig].includes(status.toUpperCase()),
  },
  size: {
    type: String,
    default: 'sm',
    validator: (size) => ['sm'].includes(size),
  },
  position: {
    type: String,
    default: 'left',
  },
  autoFlip: {
    type: Boolean,
    default: true,
  },
  triggerEvent: {
    type: String,
    default: 'hover',
    validator: (value) => ['click', 'hover'].includes(value),
  },
  hoverHideDelay: {
    type: Number,
    default: 300,
  },
  gap: {
    type: Number,
    default: 10,
  },
  disabled: {
    type: Boolean,
    default: false,
  },
  escToClose: {
    type: Boolean,
    default: false,
  },
});

const visible = ref(false);

const rootRef = useTemplateRef('rootRef');

function open() {
  if (props.disabled || visible.value) return;

  visible.value = true;
}

function close() {
  if (props.disabled || !visible.value) return;

  visible.value = false;
}

// <editor-fold desc="Handling events">
const triggerEvents = {
  hover: 'hover',
  click: 'click',
};
let hideTimeout = null;

// click
function onTriggerClick() {
  if (props.triggerEvent !== triggerEvents.click) return;

  visible.value ? close() : open();
}

onMounted(() => {
  if (props.triggerEvent === triggerEvents.click) {
    onClickOutside(rootRef, close);
  }
});

// hover
function onPointerEnter() {
  if (props.triggerEvent === triggerEvents.hover) {
    clearTimeout(hideTimeout);
    open();
  }
}

function onPointerLeave() {
  if (props.triggerEvent === triggerEvents.hover) {
    hideTimeout = setTimeout(close, props.hoverHideDelay);
  }
}

// escape
function onEsc() {
  if (visible.value && props.escToClose) close();
}
// </editor-fold>

defineExpose({ open, close, visible });
</script>

<template>
  <div
    v-if="status"
    ref="rootRef"
    class="ui-status"
    :class="{
      [`ui-status--size--${size}`]: !!size,
      [`ui-status--status--${status.toLowerCase()}`]: !!status,
    }"
    @pointerenter="onPointerEnter"
    @pointerleave="onPointerLeave"
    @keydown.esc="onEsc"
    @click="onTriggerClick"
  >
    <AFade>
      <CFloating
        v-if="visible"
        class="ui-status__content"
        :reference-ref="rootRef"
        :position="position"
        :auto-flip="autoFlip"
        :gap="gap"
      >
        <p class="i2-r-r">
          {{ status?.toLowerCase() }}
        </p>
      </CFloating>
    </AFade>
  </div>
</template>

<style lang="scss" scoped>
.ui-status {
  pointer-events: auto;
  background-color: $accent-color-warning;
  border-radius: 50%;
  transition: background-color $time-normal $ease;

  &__content {
    padding: em(5) em(10);
    text-transform: capitalize;
    background-color: rgba($background-color-primary, 1);
    border: 1px solid $border-color-secondary;
    border-radius: em(200);
  }

  &--size {
    &--sm {
      width: em(13);
      height: em(13);
    }
  }

  &--status {
    &--running,
    &--installed {
      background-color: $accent-color-success;
    }

    &--terminated {
      background-color: $accent-color-error;
    }
  }
}
</style>
