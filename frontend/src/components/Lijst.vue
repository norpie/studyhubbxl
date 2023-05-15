<template>
    <div class="address-list-wrapper">
      <div class="address-list">
        <slot></slot>
      </div>
    </div>
  </template>
  
  <script lang="ts">
  export default {
    name: "Lijst",
  };
  </script>
  
  <style scoped>
  .address-list-wrapper {
    height: 330px;
    overflow-y: scroll;
    display: flex;
    justify-content: center;
  }
  
  .address-list {
    width: 200px;
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
  }
  </style>