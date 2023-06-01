<template>
    <div class="logo-panel-container" id="logo-panel-container" :class="{ 'expanded': isExpanded }">
        <div class="logo-panel" @click="toggleExpansion">
            <img :src="logoSrc" alt="Logo" class="logo" />
        </div>
        <div class="horizontal-bar-wrapper" v-if="isExpanded">
            <div class="horizontal-bar" id="horizontal-bar">
                <Faq />
                <Privacy />
                <Panel class="Login" id="login" v-if="!isLoggedIn() && !isSignUpVisible && !isForgot" label="Login">
                    <br>
                    <br />
                    <h1>Login</h1>
                    <hr class="black-bar" />
                    <Textfield id="email" label="Mail" placeholder="Enter your email" />
                    <br />
                    <Textfield type="password" id="password" label="Password" placeholder="Enter your password" />
                    <br />
                    <Checkbox id="keepmeloggedin" label="Keep me logged in!" />
                    <br />
                    <Button label="LOGIN IN" @click="login" />
                    <br />
                    <a @click="toggleForgot" href="#">Forgot password?</a>
                    <br />
                    <br />
                    <p>Not signed up yet?</p>
                    <Button label="Sign Up!" @click="toggleSignUp" />
                </Panel>
                <Panel class="Signup" id="signup" v-if="!isLoggedIn() && isSignUpVisible && !isForgot" label="Sign Up">
                    <br />
                    <h1>Sign Up</h1>
                    <hr class="black-bar" />
                    <Textfield id="email-signin" label="Email" placeholder="Enter your email" />
                    <br />
                    <Textfield id="password-signin" label="Password" placeholder="Enter your password" type="password" />
                    <br />
                    <Textfield id="confirm-password" label="Confirm Password" placeholder="Confirm Password"
                        type="password" />
                    <br />
                    <Button label="Sign up" @click="signUp" />
                    <br />
                    <p>Already have an account? <a href="#" @click="toggleSignUp">Log in</a></p>
                </Panel>
                <Panel class="ForgotPassword" id="forgotpassword" label="Forgot Password"
                    v-if="!isLoggedIn() && !isSignUpVisible && isForgot">
                    <br />
                    <Textfield id="emailforgotpassword" label="email" />
                    <Button label="Reset Password" @click="forgot" />
                </Panel>
                <Panel class="Account" label="Account" v-if="isLoggedIn()">
                    <h1>Account</h1>
                    <hr class="black-bar" />
                    <Button id="change-password" label="Change Password" />
                    <br>
                    <Button id="deleteaccount" label="Delete Account" />
                    <br>
                    <Button id="logout" label="Logout" @click="logout" />
                </Panel>
            </div>
        </div>
    </div>
</template>

<script lang="ts">
import { loginpost, post } from "@/fetch";
import Faq from './Faq.vue';
import Privacy from './Privacy.vue';
import Panel from './Panel.vue';
import Checkbox from './Checkbox.vue';
import Textfield from './Textfield.vue';
import Button from './Button.vue';

import VueCookies from 'vue-cookies'
import { store } from '@/store'

export default {
    props: {
        logoSrc: {
            type: String,
            default: '/logo.png'
        }
    },

    data() {
        return {
            isExpanded: false,
            isSignUpVisible: false,
            isUserLoggedin: false,
            isForgot: false,
        };
    },
    methods: {
        isLoggedIn() {
            let value = $cookies.get("session");
            return value != null;
        },
        async login() {
            let email = valueOf("email");
            let password = valueOf("password");
            let keepmeloggedin = checked("keepmeloggedin");

            let body = {
                "email": email,
                "password": password
            };
            try {
                let result = await loginpost<string>("http://localhost:8080/api/v1/users/auth/login", body);
                if (keepmeloggedin) {
                    $cookies.set("session", result, "7d");
                } else {
                    $cookies.set("session", result, "session");
                }
                store.loggedIn = true;
                this.toggleExpansion();
            } catch (error) {
                console.log(error);
            }
        },

        async signUp() {
            let email = valueOf("email-signin");
            let password = valueOf("password-signin");
            let confirmpassword = valueOf("confirm-password");
            if (password != confirmpassword) {
                return (console.error("Passwords not matching")); //TODO: Error
            }
            let body = {
                "email": email,
                "password": password
            };
            try {
                let result = await loginpost<string>("http://localhost:8080/api/v1/users/auth/register", body);
                $cookies.set("session", result, "session");
                store.loggedIn = true;
                this.toggleExpansion();
            } catch (error) {
                console.log(error);
            }
        },
        logout() {
            $cookies.remove("session");
            store.loggedIn = false;
            this.toggleExpansion();
        },
        async forgot() {
            let emailforgotpassword = valueOf("emailforgotpassword");
            let result = await post<Object, String>("http://localhost:8080/api/v1/users/misc/reset", {
                "email": emailforgotpassword,
            });
            this.toggleForgot();
        },

        toggleExpansion() {
            this.isExpanded = !this.isExpanded;
        },
        toggleSignUp() {
            this.isSignUpVisible = !this.isSignUpVisible;
            if (this.isSignUpVisible) {
                clickId("signup");
            } else {
                clickId("login");
            }
        },

        toggleForgot() {
            this.isForgot = !this.isForgot;
            if (this.isForgot) {
                clickId("forgotpassword");
            }
        }
    },

    components: {
        Privacy,
        Faq,
        Panel,
        Checkbox,
        Textfield,
        Button,
    },
};

function checked(id: string) {
    let element = document.getElementById(id);
    if (element == null) {
        return;
    }
    if (!(element instanceof HTMLInputElement)) {
        return;
    }
    return element.checked;
}

function valueOf(id: string) {
    let element = document.getElementById(id);
    if (element == null) {
        return "";
    }
    if (!(element instanceof HTMLInputElement)) {
        return;
    }
    return element.value;
}

function clickId(id: string) {
    setTimeout(() => {
        let element = document.getElementById(id);
        if (element == null) {
            return;
        }
        let child = element.children[0];
        if (child == null) {
            return;
        }
        if (!(child instanceof HTMLElement)) {
            return;
        }
        child.click();
    }, 1);
}
</script>


<style scoped>
.Login {
    opacity: 2;
    width: 33%;
}

.Login.expanded {
    opacity: 2;
    display: flex;

    width: 450px;
}

.Signup {
    opacity: 2;
    width: 33%;
}

.Signup.expanded {
    opacity: 2;
    width: 450px;
}

.Account {
    opacity: 2;
    width: 33%;
}

.Account.expanded {
    opacity: 2;
    width: 450px;
}

.ForgotPassword {
    opacity: 2;
    width: 33%;
}

.ForgotPassword.expanded {
    opacity: 2;
    width: 450px;

}

.logo-panel-container {
    position: fixed;
    top: 10px;
    right: 10px;
    display: flex;
    justify-content: flex-end;
    align-items: center;
    height: 200px;
    transition: transform 0.3s ease-in-out;
    margin-top: -50px;
}

.black-bar {
    border: none;
    height: 2px;
    background-color: black;
}

.expanded {
    transform: translateX(-0%);
}

.logo-panel {
    border: 1px solid black;
    width: 75px;
    height: 75px;
    border-radius: 50%;
    background-color: #34BE82;
    display: flex;
    justify-content: center;
    align-items: center;
    border-width: 1px;
    border-color: black;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.2);
    cursor: pointer;
}

.logo {
    width: 80px;
    height: 80px;
}

.horizontal-bar-wrapper {
    position: relative;
    margin-left: 10px;
}

.horizontal-bar {
    border-radius: 8px;
    display: flex;
    border: 2px solid black;
    width: 600px;
    height: 40px;
    background-color: #34BE82;

}
</style>
