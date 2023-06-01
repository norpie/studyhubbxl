<script lang="ts">
import Panel from './Panel.vue';

export default {
    components: {
        Panel
    }
}
</script>

<template>
    <Panel class="Faq" label="Faq">
        <p>Your account</p>

        <p>How do I change my account email address?</p>
        <p>
            Within…, it’s not possible to change your email address. If you
            want to link a different email address to your account, you will
            have to create a whole new account.
        </p>

        <p>How do I get a new password?</p>
        <p>
            Tap "forgot password" and insert the email address linked to your
            account. Follow the link sent to that email and choose a new
            password.
        </p>

        <p>Use</p>

        <p>Is this every study place in Brussels?</p>
        <p>
            No, unfortunately, our list does not include every study place in
            Brussels, but we are actively adding places to our website.
        </p>

        <p>Is this site only for students?</p>
        <p>
            No, of course not! Everyone is free to use our website and find
            study places. However, only students are able to create an account
            within the website and create a favorite list.
        </p>
    </Panel>
</template>

<style scoped>
.Faq {
    opacity: 2;
    width: 33%;
}

.Faq.expanded {
    opacity: 2;
    width: 450px;
}
</style>
