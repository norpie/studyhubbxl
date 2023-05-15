
<script lang="ts">
export default {
    props: {
        label:String,
        id: String,
        placeholder:String,
    },
 }

</script>


<template>
<label for="textfield">{{ label }}</label> 
<input type="text"  class="textfield" :id="id" :placeholder="placeholder" /> 
</template>

<style scoped>
label{
    font-weight: bold;
    width: 100%;
}
.textfield{ 
    color: gray;
    border-style: none;
    background-color:white;
    display: block;
    
}
</style>