<template>
    <div class="list-wrapper" :id="id">
        <div class="list">
            <slot></slot>
        </div>
    </div>
</template>

<script lang="ts">
export default {
    props: {
        id: String
    }
};
</script>

<style scoped>
.list-wrapper {
    height: 60vh;
    overflow-y: scroll;
    justify-content: center;
}

.list {
    width: 100%;
    flex-wrap: nowrap;
    flex-direction: column;
}
</style>
