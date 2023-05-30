
<script lang="ts">
export default {
    props: {
        label: String,
        id: String
    },
    data() {
        return {
            checked: false
        }
    }
}
</script>

<template>
    <div class="checkbox-wrapper">
        <label for="checkbox">{{ label }}</label>
        <input type="checkbox" class="checkbox" :id="id" v-model="checked" />
    </div>
</template>

<style scoped>
.checkbox-wrapper {
    display: inline;
}

.checkbox:checked {
    background-color: black;
    appearance: none;
}

.checkbox {
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
