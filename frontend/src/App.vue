<script setup lang="ts">
import MapBox from "./components/Mapbox.vue"
import FilterPanel from "./components/FilterPanel.vue"
import SearchPanel from "./components/SearchPanel.vue"
import FavouritesPanel from "./components/FavouritesPanel.vue"
import NavPanel from "./components/NavPanel.vue"

import { VueCookieNext } from 'vue-cookie-next'
import { store } from '@/store'

function loggedIn() {
    return VueCookieNext.getCookie('session') != null;
}

store.loggedIn = loggedIn();
</script>

<template>
    <MapBox></MapBox>
    <div class="column">
        <FilterPanel label="Filters" />
        <SearchPanel label="search" />
        <FavouritesPanel v-if="store.loggedIn" />
    </div>
    <div class="column"></div>
    <div class="column">
        <NavPanel />
    </div>
</template>

<style>
@import url('https://fonts.googleapis.com/css2?family=Suez+One&display=swap');

* {
    font-family: 'Quicksand', sans-serif;
}

.column {
    float: left;
    width: 50%;
}

/* Clear floats after the columns */
.row:after {
    content: "";
    display: table;
    clear: both;
}

html {
    width: 100%;
    height: 100%;
}

body {
    width: 100%;
    height: 100%;
    margin: 0 0 0 0;
}

*::-webkit-scrollbar {
    display: none;
}
</style>
