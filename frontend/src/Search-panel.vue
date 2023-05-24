
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
            <span class="search-icon" @click="search"></span>
        </div>
        <div class="search-results" @scroll="handleScroll">
            <ul>
                <li v-for="result in visibleResults" :key="result">{{ result }}</li>
            </ul>
        </div>
    </div>
</template>
<script>
export default{
data (){
    return {
        searchQuery:'',
        result: [],
        visibleResults: [],
        scrollOffset: 0,
        resultPerPage: 10,
        debounceTimer: null,
    };
},
methods: {
    search(){
        
    },
    handleInput(){
        clearTimeout(this.debounceTimer);
        this.debouncerTimer = setTimeout(() => {
            this.search();
        }, 1000);
    },
    handleScroll(){
        const container = document.querySelector('.search-results');
        if (container.scrollTop + container.clientHeight >= container.srcollHeight){
            this.loadMoreResults();
        }
    },
    loadMoreResults(){
        const end = this.scrollOffset + this.resultsPerPage;
        this.visibleResult = this.results.slice(0, end);
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
