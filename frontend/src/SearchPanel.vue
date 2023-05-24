
/*
-textbalk
- een search icontje
-na x-aantal sec. word er aan backend iets gevraagd 
-scroll functie -> 10 result op een panel 
-typscript
*/

<template>
    <div>
        <div class="search-bar">
            <input type="text" v-model="searchQuery" @input="handleInput" placeholder="Search...">

        </div>
        <div class="search-results" @scroll="handleScroll">
            <ul>
                <li v-for="result in visibleResults" :key="result">{{ result }}</li>
            </ul>
        </div>
    </div>
</template>
<script lang="ts">

export default {
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
            clearTimeout(this.debounceTimer);
            this.debounceTimer = setTimeout(() => {
                this.loadMoreResults();
            }, 1000);
        },
        handleScroll() {
            const container = document.querySelector('.search-results');
            if (container == null) {
                console.log("wrong");
                return;
            }
            if (container.scrollTop + container.clientHeight >= container.scrollHeight) {
                this.loadMoreResults();
            }
        },
        loadMoreResults() {
            const end = this.scrollOffset + this.resultsPerPage;
            this.visibleResults = this.results.slice(0, end);
            this.scrollOffset += this.resultsPerPage;
        },
    },
};
</script>
<style>
.search-bar {
    display: flex;
    align-items: center;
}

.search-icon {
    align-items: end;
}

.search-results {
    max-height: 300px;
    overflow-y: auto;
}
</style>
