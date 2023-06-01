<template>
    <mapbox-map :accessToken="accessToken" :center="center" :zoom="zoom" :maxBounds="bounds" :auto-resize="true">
        <MapboxMarker v-for="location in store().results" :long="tofloat(location.long)" :lat="tofloat(location.lat)">
            <Card class="card-loc" :key="location.identifier" :name="location.name" :noise="location.noise"
                :loc_type="location.location_type" :attributes="location.attributes" :id="location.identifier"
                :address="location.address" :long="location.long" :lat="location.lat" />
        </MapboxMarker>
    </mapbox-map>
</template>

<script lang="ts">
import type { Map } from 'mapbox-gl';
import MapboxMarker from './MapboxMarker.vue'
import Card from './Card.vue'
import { store } from "@/store"

export default {
    components: {
        MapboxMarker,
        Card
    },
    methods: {
        tofloat(input: String) {
            return Number(input);
        },
        store() { return store }

    },
    data() {
        return {
            accessToken: import.meta.env.VITE_MAPBOX,
            center: [4.363567, 50.844674],
            zoom: 11.5,
            bounds: [
                [4.15, 50.75], [4.6, 50.95]
            ]
        }
    }
}
</script>

<style scoped>
.mapbox-map {
    position: fixed;
    top: 0;
    left: 0;
}
</style>
