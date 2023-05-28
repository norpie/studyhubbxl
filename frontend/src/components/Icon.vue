<template>
  <div class="icon-container">
    <img :src="src">
  </div>
</template>

<script lang="ts">
export default {
  props: {
    src: String,
  }
};

</script>

<style scoped>
.icon-container {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 15px;
  height: 15px;
  margin-left: 5px;

}

.icon-container img {
  width: 100%;
  height: 100%;
  object-fit: contain;
}
</style>