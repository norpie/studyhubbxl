<script lang="ts">
export default {
    props: {
        label: String,
        
    },
}

</script>

<template>
    <button >{{ label }}</button>
</template>

<style scoped>
button {
    font-weight: bold;
    color: white;
    background-color: blue;
    opacity: 60%;
    border: none;
    padding: 5px 20px 5px 20px;
    border-radius: 8px;
}

button:active {
    background-color: blueviolet;

}
</style>


