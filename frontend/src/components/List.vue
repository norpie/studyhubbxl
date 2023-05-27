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
    height: 300px;
    overflow-y: scroll;
    justify-content: center;
}

.list {
    width: 100%;
    display: flex;
    flex-wrap: nowrap;
    flex-direction: column;
}
</style>
