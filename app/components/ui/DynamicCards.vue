<script setup>
import sectionFlow from '~/configs/sectionFlow';

defineProps({
  title: {
    type: String,
    default: '',
  },
  items: {
    type: Array,
    default: () => [],
  },
  pending: {
    type: Boolean,
    default: false,
  },
  error: {
    type: Error,
    default: undefined,
  },
});

const contentFlow = ref(sectionFlow.grid);

const stopWatcher = watch(
  () => contentFlow.value,
  () => {
    const { $flip } = useNuxtApp();
    const flipState = $flip.getState(
      '.ui-dynamic-cards__content, .ui-universal-card',
    );

    nextTick(() => {
      $flip.from(flipState, {
        duration: 0.25,
        ease: 'power0.inOut',
      });
    });
  },
);

onBeforeUnmount(stopWatcher);

const emits = defineEmits(['update:contentFlow', 'refresh']);
</script>

<template>
  <div class="ui-dynamic-cards">
    <UiDynamicSectionHeader
      :title="title"
      :pending="pending"
      :model-value="contentFlow"
      @update:model-value="contentFlow = $event"
      @fetch="emits('refresh')"
    />

    <UiError v-if="error" :error="error" />

    <div
      class="ui-dynamic-cards__content"
      :class="{
        [`ui-dynamic-cards__content--${contentFlow}`]: true,
      }"
    >
      <TransitionGroup name="cards">
        <template v-if="$slots.default">
          <div
            v-for="(item, index) in items?.length ? items : 8"
            :key="index"
            class="cards-item"
          >
            <slot
              :data="{
                ...item,
                pending: !(items?.length || !pending) && !error,
              }"
            />
          </div>
        </template>
      </TransitionGroup>
    </div>
  </div>
</template>

<style scoped lang="scss">
.ui-dynamic-cards {
  display: flex;
  flex-direction: column;
  gap: em(20);

  &__content {
    display: grid;
    gap: em(20);

    &--grid {
      grid-template-columns: repeat(6, 1fr);

      @include media-breakpoint-down(xxl) {
        grid-template-columns: repeat(4, 1fr);
      }

      @include media-breakpoint-down(lg) {
        grid-template-columns: repeat(3, 1fr);
      }

      @include media-breakpoint-down(md) {
        grid-template-columns: repeat(2, 1fr);
      }

      @include media-breakpoint-down(sm) {
        grid-template-columns: 1fr;
      }
    }
  }
}
</style>
