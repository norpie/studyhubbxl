<template>
    <div class="card">
        <div class="card-rectangle">{{ label }}
        <slot></slot>
        </div>
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
    width: 90%;
    height: 100px;
    margin: 10px;
    display: flex;
    justify-content: center;
    align-items: center;
}

.card-rectangle {
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
