<template>
    <mapbox-marker :lngLat="[long, lat]" :draggable="false">
        <mapbox-popup>
            <slot></slot>
        </mapbox-popup>
    </mapbox-marker>
</template>

<script lang="ts">
export default {
    props: {
        long: Number,
        lat: Number,
    }
}
</script>

<style scoped></style>
