
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
        <input type="checkbox" id="checkbox" v-model="checked" />
    </div>
</template>



<style scoped>
input:checked {
    accent-color: grey;
    background-color: black;
    height: 10px;
    border-radius: 2px;
    width: 10x;
    appearance: none;
}

input {
    accent-color: grey;
    background-color: white;
    border-color: black;
    border-style: solid;
    border-radius: 2px;
    border-width: 1px;
    margin: 0px;
    margin-left:5px ;
    height: 10px;
    width: 10px;
    appearance: none;

}

label {

    align-items: center;
}
</style>



