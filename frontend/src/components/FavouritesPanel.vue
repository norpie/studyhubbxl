<template>
    <Panel label="favourites">
        <List class="list" @scroll="handleScroll">
            <Card class="card-loc" v-for="location in store().favourites" :key="location.identifier" :name="location.name"
                :noise="location.noise" :loc_type="location.location_type" :attributes="location.attributes"
                :id="location.identifier" :address="location.address" :long="location.long" :lat="location.lat" />
        </List>
    </Panel>
</template>

<style scoped>
.list::-webkit-scrollbar {
    display: none;
}
</style>
<script lang="ts">

import Panel from './Panel.vue';
import Icon from './Icon.vue';
import Card from './Card.vue';
import List from './List.vue';
import { get } from '../fetch';
import { store } from '../store';
import type Location from '../models/location';

export default {
    components: { Icon, Panel, Card, List },

    data() {
        return {
            searchQuery: '',
            results: [] as any[],
            visibleResults: [] as Location[],
            scrollOffset: 0,
            resultsPerPage: 10,
            debounceTimer: 0,
            fetching: false,
        };
    },
    mounted() {
        this.loadMoreResults();
    },
    methods: {
        handleScroll() {
            const container = document.querySelector('.list');
            if (container == null) {
                return;
            }
            const end = this.scrollOffset + this.resultsPerPage;
            this.visibleResults = store.favourites.slice(0, end);
            this.scrollOffset += this.resultsPerPage;
            if (container.scrollTop + container.clientHeight >= container.scrollHeight) {
                this.loadMoreResults();
            }
        },
        store() {
            return store;
        },
        async loadMoreResults() {
            if (this.fetching) {
                return;
            }
            this.fetching = true;
            try {
                let url = 'http://localhost:8080/api/v1/users/favourites?limit=10&start=' + store.favourites.length;
                const response = await get<Location[]>(url, {
                    mode: "cors",
                    credentials: "include"
                });
                store.favourites = store.favourites.concat(response);
            } catch (error) {
                console.error(error);
            }
            this.fetching = false;
        },
    }
};

</script>
