<template>
    <div>
        <p>Id: {{ id }}</p>
        <p>Email: {{ email }}</p>
        <p>Username: {{ username }}</p>
        <p>Password: {{ password }}</p>
        <p>Salt: {{ salt }}</p>
        <p>Error: {{ error }}</p>
    </div>
</template>

<script lang="ts">

import { get } from "@/fetch";
import type User from "@/models/user";

export default {
    data() {
        return {
            id: '',
            email: '',
            username: '',
            password: '',
            salt: '',
            error: ''
        };
    },
    async mounted() {
        try {
            let user = await get<User>("http://localhost:8080");
            this.id = user.id;
            this.email = user.email;
            this.username = user.username;
            this.password = user.password;
        } catch (error) {
            if (error instanceof Error) {
                this.error = error.message;
                console.log(error);
            }
        }
    },
};
</script>
