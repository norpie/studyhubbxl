<template>
    <mapbox-marker :lngLat="[lon, lat]" :draggable="false">
        <mapbox-popup>
            <slot></slot>
        </mapbox-popup>
    </mapbox-marker>
</template>

<script lang="ts">
export default {
    props: {
        lon: Number,
        lat: Number,
    }
}
</script>

<style scoped></style>
