<script setup>
const data = ref();
const init = async () => (data.value = await initGcloud());
</script>

<template>
  <div class="page">
    <div class="container">
      <button
        style="
          padding: 10px 20px;
          color: #ffffff;
          background-color: #333333;
          border-radius: 8px;
        "
        @click="init"
      >
        Authorize GCloud
      </button>
      <div v-if="data?.message" style="color: green">{{ data.message }}</div>
      <div v-if="data?.error" style="color: red">{{ data.error }}</div>
      <div v-if="data?.data">{{ data.data }}</div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.page {
  padding-block: 20px;
}

.container {
  display: flex;
  flex-direction: column;
  gap: 10px;
  align-items: flex-start;
}
</style>
