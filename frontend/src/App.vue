<script setup lang="ts">
import Button from './components/Button.vue'

function login() {
    
}

function logout() {
    
}

function delAccount() {
    
}

function forgotPsw() {
    
}

function changePsw() {
    
}


</script>

<template>
    
    <Button label="login" @event="login"/>
    <br>
    <Button label = "logout" @event="logout"/>
    <br>
    <Button label = "delete account" @event="delAccount"/>
    <br>
    <Button label = "forgot passwoord" @event="forgotPsw"/>
    <br>
    <Button label = "change passwoord " @event="changePsw"/>
</template>



