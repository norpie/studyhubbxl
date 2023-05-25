<script lang="ts">

export default {
    components: { Icon, Panel, Card, List }

}
import Panel from './Panel.vue';
import Icon from './Icon.vue';
import Card from './Card.vue';
import List from './List.vue';
</script>

<template>
    <Panel label="Favourites" style="">
        <List class="List">
            <Card class="card-fav" label="je moeder" >
                <Icon src="search-icon.png" />
            </Card>
            <Card class="card-fav" label="je vader">
                <Icon src="search-icon.png" />
            </Card>
            <Card class="card-fav" label="je broer">
                <Icon src="search-icon.png" />
            </Card>
            <Card class="card-fav" label="je zus">
                <Icon src="search-icon.png" />
            </Card>
            <Card class="card-fav" label="je stief zus">
                <Icon src="search-icon.png" />
            </Card>
            <Card class="card-fav" label="je stief broer">
                <Icon src="search-icon.png" />
            </Card>
            <Card class="card-fav" label="je stief vader">
                <Icon src="search-icon.png" />
        </Card>
        <Card class="card-fav" label="je stief moeder">
            <Icon src="search-icon.png" />
        </Card>
    </List>
</Panel></template>

<style scoped>
.List::-webkit-scrollbar{

}

</style>