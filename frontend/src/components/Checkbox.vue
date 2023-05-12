
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
    <div>
        <label for="checkbox">{{ label }}</label>
        <input type="checkbox" class="checkbox" :id="id" v-model="checked" />
    </div>
</template>

<style scoped>
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
    margin: 0px;
    margin-left: 5px;
    height: 10px;
    width: 10px;
    appearance: none;
}
</style>
