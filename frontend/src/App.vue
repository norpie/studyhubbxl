<script setup lang="ts">
import MapBox from "./components/Mapbox.vue"
import FilterPanel from "./components/FilterPanel.vue"
import SearchPanel from "./components/SearchPanel.vue"
import FavouritesPanelVue from "./components/FavouritesPanel.vue"
import NavPanelVue from "./components/NavPanel.vue"
</script>

<template>
    <MapBox></MapBox>
    <FilterPanel label="Filters" />
    <SearchPanel label="search" />
    <FavouritesPanelVue />
    <NavPanelVue />
</template>

<style>
@import url('https://fonts.googleapis.com/css2?family=Suez+One&display=swap');

* {
    font-family: 'Quicksand', sans-serif;
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
