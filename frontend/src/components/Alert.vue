<script lang="ts">
export default {
  props:{
    label:String,
    style:String
  }
};

import Icon from './Icon.vue';
</script>
<template>
  <div class="container" :style=style >
    <div class="icon">
      <img class="img-icon" src="/search-icon.png"/>
    </div>
    <div class="titel">
        <h2 class="text">{{ label }}</h2>
    </div>
  </div>
</template>

<style>

div.container{
  display: flex;
  align-items: center;
  max-width: 20%;
  transform: translateX(175%);
  border-radius: 10px;
  margin: 10px;
  padding: 20px;
  opacity: 80%;
}
div.icon{
  color: brown;
}
div.titel{
  flex: 1;
}
h2.text{
 margin: 0;
  font: medium;
  text-align: center;
  font-size:20px
}
.img-icon{
  max-width: 25px;
}
</style>
