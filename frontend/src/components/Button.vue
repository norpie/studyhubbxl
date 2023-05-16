<template>
    <button @click="$emit('click')">{{ label }}</button>
</template>

<style scoped>
button {
    font-weight: bold;
    color: white;
    background-color: grey;
    opacity: 60%;
    border: none;
    padding: 5px 20px 5px 20px;
    border-radius: 10px;
    margin: 5px;
}

button:active {
    background-color: brown;

}
</style>

<script lang="ts">

export default {
    props: {
        label: String,
    },
}
</script>
