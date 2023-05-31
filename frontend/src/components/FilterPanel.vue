<script lang="ts">

import Panel from './Panel.vue'
import Checkbox from './Checkbox.vue'
import Icon from './Icon.vue'
import { get } from "../fetch";
import type FilterItem from "../models/filteritem";
import { showResults } from "@/results";


export default {
  async mounted() {
    try {
      this.items_place = await get<FilterItem[]>('http://localhost:8080/api/v1/filters/location_type');
      this.items_attributes = await get<FilterItem[]>('http://localhost:8080/api/v1/filters/attribute');
      this.items_volume = await get<FilterItem[]>('http://localhost:8080/api/v1/filters/noise');
    }
    catch (error) {
      console.error(error);;
    }
    //http://localhost:8080/api/v1/filters/location_types
  },
  data() {
    return {
      items_place: [] as FilterItem[] /*[
        { display_name: 'Cafe', path: 'cafe-icon.png' },
        { display_name: 'Studyspaces', path: 'studyspace-icon.png' },
        { display_name: 'Campus', path: 'campus-icon.png' },
        { display_name: 'Library', path: 'library-icon.png' }
      ]*/,
      items_attributes: [] as FilterItem[] /*[
        { display_name: 'Sockets', path: 'socket-icon.png' },
        { display_name: 'Wifi', path: 'wifi-icon.png' },
        { display_name: 'Coworking space', path: 'coworking-icon.png' },
        { display_name: 'Free to access', path: 'free-icon.png' }
      ]*/,
      items_volume: [] as FilterItem[] /*[
        { display_name: 'Noisy', path: 'noisy-icon.png' },
        { display_name: 'Moderate', path: 'moderate-icon.png' },
        { display_name: 'Quiet', path: 'quiet-icon.png' },
        { display_name: 'Silent', path: 'silent-icon.png' }
      ]*/,
      selectedPlaces: [], // voeg plekken
      selectedAttributes: [], // voeg attributen
      selectedVolume: [], // voeg volume
    }
  },
  components: {
    Panel,
    Checkbox,
    Icon,
  },
  methods: {
    /*search() {
      // die here
      fetchResults
      (this.selectedPlaces, this.selectedPlaces, this.selectedVolume)
        .then(results => {
          this.seacrhResults = results;
        })
        .catch(error => {
          console.error('Error fecthing results', error);
        })
    }*/
    search() {
      showResults(0, true);
    },
  },
};
</script>

<template>
  <Panel label="Filter">
    <p>Places</p>
    <p v-for="item in items_place">
      <Checkbox :label="item.display_name" />
      <Icon :src="item.path" />
    </p>
    <p>Attributes</p>
    <p v-for="item in items_attributes">
      <Checkbox :label="item.display_name" />
      <Icon :src="item.path" />
    </p>
    <p>Volume</p>
    <p v-for="item in items_volume">
      <Checkbox :label="item.display_name" />
      <Icon :src="item.path" />
    </p>
  </Panel>
</template>

<script>
</script>
