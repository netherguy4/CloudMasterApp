<template>
  <div class="ui-skeleton-block" />
</template>

<style lang="scss" scoped>
@keyframes pulse {
  0% {
    background-position: 25% 0;
  }

  100% {
    background-position: 75% 0;
  }
}

.ui-skeleton-block {
  height: em(45);
  background: linear-gradient(
    90deg,
    rgb(255 255 255 / 0%) 0%,
    rgb(255 255 255 / 10%) 50%,
    rgb(255 255 255 / 0%) 100%
  );
  background-size: 400% 100%;
  border-radius: em(8);
  animation: pulse 1.5s $ease-in-out infinite alternate;
}
</style>
