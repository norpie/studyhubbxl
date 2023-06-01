
<script lang="ts">
import Icon from './Icon.vue';
export default {
    props: {
        label: String,
        id: String,
        src: String,
        class: String,
        checked: Boolean
    },
    components: {
        Icon
    }
}
</script>

<template>
    <div class="checkbox-wrapper">
        <input type="checkbox" :class="class" :id="id" :checked="checked" />
        <Icon :src="src" />
        <label for="checkbox">{{ label }}</label>
    </div>
</template>

<style scoped>
.checkbox-wrapper {
    display: inline;
}

input[type='checkbox']:checked {
    background-color: black;
    appearance: none;
}

input[type='checkbox'] {
    accent-color: grey;
    background-color: white;
    border-color: black;
    border-style: solid;
    border-radius: 2px;
    border-width: 1px;
    margin-top: 5px;
    margin-left: 5px;
    margin-right: 5px;
    height: 10px;
    width: 10px;
    appearance: none;
    float: left;
}
</style>
