<script lang="ts">
export default {
    props: {
        label: String,
        id: String,
        placeholder: String,
        v_model: String,
        input: String
    },
}
</script>


<template>
    <label for="textfield">{{ label }}</label>
    <input type="text" class="textfield" :id="id" :placeholder="placeholder" :@input="input" :v-model="v_model" />
</template>

<style scoped>
label {
    font-weight: bold;
    width: 100%;
}

.textfield {
    color: gray;
    border-style: none;
    background-color: white;
    display: block;
    margin: 0;
    padding: 0;

}
</style>
