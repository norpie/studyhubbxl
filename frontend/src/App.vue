<script setup lang="ts">
import Button from './components/Button.vue'

function test() {
    console.log("test");
}
</script>

<template>
    <Button label="test" @click="test"/>
</template>
