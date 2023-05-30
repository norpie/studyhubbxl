<template>
    <Panel label="favourites">
        <list class="list" @scroll="handleScroll">
            <Card class="card-fav" v-for="location in results" :key="location.id" :label="location.name">
                <Icon src="search-icon.png" />
            </Card>

        </list>
    </Panel>
</template>

<style scoped>
.list::-webkit-scrollbar {
    display: none;
}
</style>
<script lang="ts">

export default {
    components: { Icon, Panel, Card, List },

    data() {
        return {
            searchQuery: '',
            results: [] as Location[],
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
                console.log("wrong");
                return;
            }
            if (container.scrollTop + container.clientHeight >= container.scrollHeight) {
                this.loadMoreResults();
            }
        },
        async loadMoreResults() {
            if (this.fetching) {
                return;
            }
            this.fetching = true;
            const end = this.scrollOffset + this.resultsPerPage;
            this.visibleResults = this.results.slice(0, end);
            this.scrollOffset += this.resultsPerPage;
            try {
                let url = 'http://localhost:8080/api/v1/users/favourites?limit=10&start=' + this.results.length;
                const response = await get<Location[]>(url);
                console.log(response);
            } catch (error) {
                console.error(error);
            }
            this.fetching = false;
        },

    }

};


import Panel from './Panel.vue';
import Icon from './Icon.vue';
import Card from './Card.vue';
import List from './List.vue';
import { get } from '../fetch';
import type Location from '../models/location';


</script>