
<script lang="ts">
export default {
    props: {label:String},

    data() {return{
        text: 'Enter credentials',
    }},
 }

</script>


<template>
<label for="textfield">{{ label }}</label> 
<input type="text"  class="textfield" v-model="text" />
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