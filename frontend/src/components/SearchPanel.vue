
<template>
    <Panel label="search">

        <input class="search-bar" type="text" v-model="searchQuery" id="search-box" @input="handleInput" placeholder="Search...">
        <div class="search-results" @scroll="handleScroll">
            <ul>
                <li v-for="result in visibleResults" :key="result">{{ result }}</li>
            </ul>
        </div>
    </Panel>
</template>
<script lang="ts">

import Panel from './Panel.vue';
import { showResults } from "@/results";

export default {
    components: {
        Panel
    },
    data() {
        return {
            searchQuery: '',
            results: [],
            visibleResults: [],
            scrollOffset: 0,
            resultsPerPage: 10,
            debounceTimer: 0,
        };
    },
    methods: {
        handleInput() {
            showResults(true);
            //clearTimeout(this.debounceTimer);
            //this.debounceTimer = setTimeout(() => {
            //    this.loadMoreResults();
            //}, 1000);
        },
        handleScroll() {
            //const container = document.querySelector('.search-results');
            //if (container == null) {
            //    console.log("wrong");
            //    return;
            //}
            //if (container.scrollTop + container.clientHeight >= container.scrollHeight) {
            //    this.loadMoreResults();
            //}
        },
        loadMoreResults() {
            //console.log("test");
            //const end = this.scrollOffset + this.resultsPerPage;
            //this.visibleResults = this.results.slice(0, end);
            //this.scrollOffset += this.resultsPerPage;
        },
        performSearch() {
            //console.log("Performing search...");
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
