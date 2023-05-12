<script lang="ts">
export default {
    props: {p:String},

    data() {return{
        mail: 'Enter your email',
        password:'Enter your password'}},
 }

</script>


<template>
<p> Mail </p>
<input type="text"  class="mail" v-model="mail" placeholder="Enter your email" />
<p> Password </p>
<input type="text"  class="password" v-model="password" placeholder="Enter your email" />
</template>

<style scoped>
p{font-weight: bold;}
.mail{ 
    color: gray;
    border-style: none;
    background-color:white;
}
.password{ 
    color:gray;
    border-style: none;
    background-color: white;
    }
</style>