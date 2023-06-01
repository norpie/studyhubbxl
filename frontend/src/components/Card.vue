<template>
    <div class="card">
        <div class="card-rectangle" :id="id">
            <p class="titel">{{ name }} </p>
            <p class="adres">{{ address }} </p>
            <div class="icon">
                <div class="attributes">
                    <Icon :src="noise_path(noise)" />
                    <Icon :src="type_path(loc_type)" />
                    <Icon v-for="item in attributes" :src="attribute_path(item)" />
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
import { delete_, post } from "@/fetch";
import { store } from "@/store";
import type Location from "@/models/location";

export default defineComponent({
    components: { Icon, ClickableIcon, Location },

    props: {
        id: String,
        label: String,
        name: String,
        address: String,
        attributes: {
            type: Array<string>
        },
        noise: String,
        loc_type: String
    },

    data() {
        return {
            currentImage: this.isFavourite() ? '/star.png' : '/star-empty.png',
        };
    },

    methods: {
        isFavourite() {
            for (let i = 0; i < store.favourites.length; i++) {
                if (store.favourites[i].identifier == this.id) {
                    return true;
                }
            }
            return false;
        },
        noise_path(noise: String) {
            for (let i = 0; i < store.noises.length; i++) {
                if (store.noises[i].identifier == noise) {
                    return store.noises[i].path;
                }
            }
            //console.log(noise);
        },
        type_path(loc_type: String) {
            for (let i = 0; i < store.types.length; i++) {
                if (store.types[i].identifier == loc_type) {
                    return store.types[i].path;
                }
            }
            //console.log(loc_type);
        },
        attribute_path(attribute: String) {
            for (let i = 0; i < store.attributes.length; i++) {
                if (store.attributes[i].identifier == attribute) {
                    return store.attributes[i].path;
                }
            }
            //console.log(attribute);
        },
        attributes_paths(attributes: String[]) {
            let paths = [];
            for (let i = 0; i < attributes.length; i++) {
                paths.push(this.attribute_path(attributes[i]));
            }
            return paths;
        },
        clickId(id: string) {
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
        },
        openLogin() {
            let horizontal = document.getElementById("horizontal-bar");
            if (horizontal == null) {
                this.clickId("logo-panel-container");
            }
            this.clickId("login");
        },
        async changeImage() {
            if (this.currentImage === '/star-empty.png') {
                try {
                    await post<string, string>('http://localhost:8080/api/v1/users/favourites/' + this.id, "", {
                        mode: "cors",
                        credentials: "include"
                    });
                    const location: Location = {
                        identifier: this.id,
                        name: this.name,
                        location_type: this.location_type,
                        attributes: this.attributes,
                        noise: this.noise,
                        address: this.address,
                        lat: this.lat,
                        long: this.long,
                    }
                    store.favourites.push(location);
                    this.currentImage = '/star.png';
                } catch {
                    this.openLogin();
                    console.log("fail.");
                }
            }
            else {
                try {
                    await delete_<string>('http://localhost:8080/api/v1/users/favourites/' + this.id, {
                        mode: "cors",
                        credentials: "include",
                    });
                    let new_favourites = [];
                    for (let i = 0; i < store.favourites.length; i++) {
                        if (store.favourites[i].identifier != this.id) {
                            new_favourites.push(store.favourites[i]);
                        }
                    }
                    store.favourites = new_favourites;
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
