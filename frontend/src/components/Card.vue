<template>
    <div class="card">
        <div class="card-rectangle">
            <p class="titel">{{ name }} </p>
            <p class="adres">{{ address }} </p>
            <ClickableIcon class="clickable-icon">
                <p class="icon" v-for="item in location_place">
                    <Icon :src="item.path" :id="item.id" />
                </p>
                <p class="icon" v-for="item in location_attributes">
                    <Icon :src="item.path" :id="item.id" />
                </p>
                <p class="icon" v-for="item in location_noise">
                    <Icon :src="item.path" :id="item.id" />
                </p>

            </ClickableIcon>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import Icon from "./Icon.vue";
import ClickableIcon from "./ClickableIcon.vue";
import type Location from "../models/location";
import type FilterItem from "@/models/filteritem";

export default defineComponent({
    components: { Icon, ClickableIcon, Location },


    props: {
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
            location_place:
                [{ id: 'cafe', path: 'cafe-icon.png' },
                { id: 'studyspaces', path: 'studyspace-icon.png' },
                { id: 'campus', path: 'campus-icon.png' },
                { id: 'library', path: 'library-icon.png' }] as FilterItem[],
            location_attributes:
                [{ id: 'sockets', path: 'socket-icon.png' },
                { id: 'wifi', path: 'wifi-icon.png' },
                { id: 'coworking space', path: 'coworking-icon.png' },
                { id: 'free to access', path: 'free-icon.png' }] as FilterItem[],
            location_noise:
                [{ id: 'noisy', path: 'noisy-icon.png' },
                { id: 'moderate', path: 'moderate-icon.png' },
                { id: 'quiet', path: 'quiet-icon.png' },
                { id: 'silent', path: 'silent-icon.png' }] as FilterItem[]
        }

    }
},

);

</script>

<style scoped>
.card {

    display: flex;
    justify-content: flex-end;
    align-items: flex-end;
    width: 200px;
    height: 100px;
    margin: 10px;

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
    justify-content: end;
    align-items: end;
    height: 25px;
    width: 25px;
    float: right;
}

.clickable-icon {}

.titel {
    margin: 0;
    float: left;
    margin-left: 2px;

}

.adres {
    float: left;
    margin: 0;
    font-weight: normal;
    margin-top: 2px;
    margin-left: 2px;
    max-width: 200px;
    column-span: 1;
    font-size: small;
}
</style>
