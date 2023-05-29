<template>
    <div class="logo-panel-container" :class="{ 'expanded': isExpanded }">
      <div class="logo-panel" @click="toggleExpansion">
        <img :src="logoSrc" alt="Logo" class="logo" />
      </div>
      <div class="horizontal-bar-wrapper" v-if="isExpanded">
        <div class="horizontal-bar"><slot></slot></div>

      </div>
    </div>
  </template>
  
  <style scoped>
  .logo-panel-container {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    height: 200px;
    transition: transform 0.3s ease-in-out;
  }
  
  .expanded {
    transform: translateX(-0%); 
  }
  
  .logo-panel {
    width: 150px;
    height: 150px;
    border-radius: 50%;
    background-color: #2F86A6;
    display: flex;
    justify-content: center;
    align-items: center;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.2);
    cursor: pointer;
  }
  
  .logo {
    width: 80px;
    height: 80px;
  }
  
  .horizontal-bar-wrapper {
    position: relative;
    margin-left: 10px;
  }
  
  .horizontal-bar {
    display: flex;
    justify-content: space-evenly;
    width: 600px;
    height: 40px;
    background-color: #ccc;
  }
  </style>
  
  <script lang="ts">
  export default {
    props: {
      logoSrc: {
        type: String,
        required: true
      }
    },
    data() {
      return {
        isExpanded: false
      };
    },
    methods: {
      toggleExpansion() {
        this.isExpanded = !this.isExpanded;
      }
    }
  };
  </script>
  