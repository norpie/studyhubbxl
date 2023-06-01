<template>
    <Panel label="search">
        <input class="search-bar" type="text" id="search-box" @input="handleInput" placeholder="Search...">
        <List class="list-search-results" @scroll="handleScroll">
            <Card class="card-loc" v-for="location in store().results" :key="location.identifier" :name="location.name"
                :noise="location.noise" :loc_type="location.location_type" :attributes="location.attributes"
                :id="location.identifier" :address="location.address" :long="location.long" :lat="location.lat" />
        </List>
    </Panel>
</template>
<script lang="ts">

import Panel from './Panel.vue';
import Card from './Card.vue';
import Icon from './Icon.vue';
import List from './List.vue';
import { showResults } from "@/results";
import { store } from "@/store";

export default {
    components: {
        Panel, List, Card, Icon
    },
    async mounted() {
        setTimeout(async () => {
            this.results = await showResults(0, true);
        }, 1000);
    },
    data() {
        return {
            results: [] as any[],
            visibleResults: [] as Location[],
            scrollOffset: 0,
            resultsPerPage: 10,
            debounceTimer: 0,
            fetching: false,
        };
    },
    methods: {
        async handleInput() {
            clearTimeout(this.debounceTimer);
            this.debounceTimer = setTimeout(async () => {
                await showResults(0, true);
            }, 300);
        },
        store() {
            return store;
        },
        async handleScroll() {
            if (this.fetching) {
                return;
            }
            this.fetching = true;
            const container = document.querySelector('.list');
            if (container == null) {
                return;
            }
            const end = this.scrollOffset + this.resultsPerPage;
            this.visibleResults = store.results.slice(0, end);
            this.scrollOffset += this.resultsPerPage;
            if (container.scrollTop + container.clientHeight >= container.scrollHeight) {
                await showResults(store.results.length, false);
            }
            this.fetching = false;
        }
    },
};
</script>
<style>
.search-bar {
    display: flex;
    align-items: center;
    justify-content: flex-start;
    margin: 10px;
    /*border-radius: 20px;
    overflow: hidden;*/
    background: none;
    width: 140px;

}

.placeholder {
    color: black;
}

.search-input {
    flex-grow: 1;
    margin: 0;
    border: none;
    border-radius: 20px;
    padding: 8px;
}


.search-results {
    max-height: 300px;
    overflow-y: auto;
}
</style>
