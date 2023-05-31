<template>
    <div class="Panel" :class="{ expanded: isExpanded }">
      <div class="header" @click="expandPanel">
        <p class="text">{{ label }}</p>
      </div>
      <div v-if="isExpanded"><slot> </slot></div>
    </div>
  </template>
  
  <style scoped>
  .Panel {
    opacity: 0.5;
    background-color: #2F86A6;
    border-radius: 10px;
    width: 33%;
    text-align: center;
    font-size: larger;
    text-transform: uppercase;
    overflow: hidden;
    transition: max-height 0.5s ease-in-out;
    border: 1px solid black;
   
  }
  
  .header {
    cursor: pointer;
  }
  
  .Panel:hover {
    background-color: #2F86A6;
  }
  
  .expanded {
      width: 400px;
      height: 50vh;
      max-height: none;
      overflow: auto;
      
  }
  .expanded::-webkit-scrollbar{
      display: none;
  }
  
  .content {
    margin-top: 10px;
  }
  </style>
  
  <script lang="ts">
  import { defineComponent } from 'vue';
  
  export default defineComponent({
      props: {
          label:String
          
      },
    data() {
      return {
        isExpanded: false,
      };
    },
  
    methods: {
      expandPanel(event: MouseEvent) {
        
        if ((event.target as HTMLElement).nodeName !== 'BUTTON') {
          this.isExpanded = !this.isExpanded;
        }
      },
    },
  });
  </script>