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
    height: 330px;
    overflow-y: scroll;
    display: flex;
    justify-content: center;
}

.list {
    width: 200px;
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
}
</style>
