
<script lang="ts">
export default {
    props: {label:String},

    data() {return{
        text: 'Enter your email',
    }},
 }

</script>


<template>
<label for="textfield">{{ label }}</label> 
<input type="text"  class="mail" v-model="text" placeholder="Enter your email" />
</template>

<style scoped>
p{font-weight: bold;}
.textfield{ 
    color: gray;
    border-style: none;
    background-color:white;
}

</style>