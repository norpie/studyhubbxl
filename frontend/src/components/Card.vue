<template>
    <div class="card">
        <div class="card-rectangle">
            <p class="titel">{{ name }} </p>
            <p class="adres">{{ address }} </p>
            <div class="icon">
                <div class="attributes">
                    <Icon :src="noise" />
                    <Icon :src="type" />
                    <Icon v-for="item in attributes" :src="item" />
                </div>
                <div class="changeicon">
                    <img :src="currentImage" @click="changeImage" alt="Image" class="image" />
                </div>
            </div>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import Icon from "./Icon.vue";
import ClickableIcon from "./ClickableIcon.vue";
import type Location from "../models/location";
import { delete_, get } from "@/fetch";

export default defineComponent({
    components: { Icon, ClickableIcon, Location },

    props: {
        id: String,
        label: String,
        name: String,
        address: String,
        attributes: {
            type: Array
        },
        noise: String,
        type: String
    },
    data() {
        return {
            currentImage: '/star-empty.png',
        };
    },
    methods: {
        async changeImage() {
            if (this.currentImage === '/star-empty.png') {
                try {
                    await get<Location[]>('http://localhost:8080/api/v1/users/favourites/' + this.id);
                    this.currentImage = '/star.png';
                } catch {
                    console.log("fail.");
                }
            }
            else {
                try {
                    await delete_<Location[]>('http://localhost:8080/api/v1/users/favourites/' + this.id);
                    this.currentImage = '/star-empty.png';
                } catch {
                    console.log("fail.");
                }
            }
        }
    }
},

);

</script>

<style scoped>
.image {
    width: 20px;
    height: 20px;
    cursor: pointer;
}

.card {

    display: flex;
    justify-content: flex-end;
    align-items: flex-end;
    width: 200px;
    height: 100px;
    margin: 10px;

}

.attributes {
    width: 180px;
}

.card-rectangle {
    display: grid;
    width: 100%;
    height: 100%;
    background-color: white;
    color: black;
    font-weight: bold;
    border-radius: 4%;
    border-color: black;
    border: solid 1px;
    grid-template-rows:
        25px 50px 25px
}

.icon {
    display: inline-flex;
    justify-content: left;
    align-items: end;
    height: 25px;
    width: 200px;
    float: left;
    flex-direction: row;

}

.titel {
    margin: 0;
    float: left;
    margin-left: 2px;

}

.adres {
    float: right;
    margin: 0;
    font-weight: normal;
    margin-top: 2px;
    margin-left: 2px;
    max-width: 200px;
    column-span: 1;
    font-size: small;

}

.button {
    background: "star-empty.png";
    width: 20px;
    height: 20px;

}

.img {
    display: flex;
    width: 20px;
    height: 20px;
    align-items: center;
    justify-content: center;
}
</style>
