<template>
    <mapbox-map :accessToken="accessToken" :center="center" :zoom="zoom" :maxBounds="bounds" :auto-resize=true>
        <slot>
        </slot>
        <!--<mapbox-navigation-control position="top-left"  />-->
        <!--<mapbox-geolocate-control />-->
    </mapbox-map>
</template>

<script lang="ts">
export default {
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
.mapbox-map{
    position: absolute;
    z-index: -1;
}
</style>
