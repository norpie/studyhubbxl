<template>
    <div>
        <p>{{ data }}</p>
    </div>
</template>

<script lang="ts">

export default {
    data() {
        return {
            data: '',
        };
    },
    async mounted() {
        try {
            const response = await fetch("http://localhost:8080", {});
            const data = await response.json();
            this.data = data;
        } catch (error) {
            if (error instanceof Error) {
                this.data = error.message;
            }
            console.log(error);
        }
    },
};
</script>
