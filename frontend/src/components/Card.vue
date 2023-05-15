<template>
    <div class="card">
        <div class="card-rectangle">{{ label }}</div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
    props: {
        label: String,
    },
});
</script>

<style scoped>
.card {
    width: 200px;
    height: 100px;
    margin: 10px;
    display: flex;
    justify-content: center;
    align-items: center;
}

.address-rectangle {
    width: 100%;
    height: 100%;
    background-color: lightgray;
    display: flex;
    justify-content: center;
    align-items: center;
    color: black;
    font-weight: bold;
}
</style>
