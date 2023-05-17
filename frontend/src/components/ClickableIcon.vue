<template>
    <button class="clickable-icon">
        <slot></slot>
    </button>
</template>

<script lang="ts">
export default {
};
</script>

<style scoped>
.clickable-icon {
    padding: 0;
    border: none;
    background: transparent;
    cursor: pointer;

}

.clickable-icon :active {
    background-color: yellow;
}

.clickable-icon img {
    height: 24px;
    width: 24px;
}
</style>
