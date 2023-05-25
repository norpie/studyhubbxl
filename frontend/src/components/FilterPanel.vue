<script lang="ts">

import Panel from './Panel.vue'
import Checkbox from './Checkbox.vue'
import Icon from './Icon.vue'

export default {
  data() {
    return {
      items_place: [
        { display_name: 'Cafe', path: 'cafe-icon.png' },
        { display_name: 'Studyspaces', path: 'studyspace-icon.png' },
        { display_name: 'Campus', path: 'campus-icon.png' },
        { display_name: 'Library', path: 'library-icon.png' }],
      items_attributes: [
        { display_name: 'Sockets', path: 'socket-icon.png' },
        { display_name: 'Wifi', path: 'wifi-icon.png' },
        { display_name: 'Coworking space', path: 'coworking-icon.png' },
        { display_name: 'Free to access', path: 'free-icon.png' }],
      items_volume: [
        { display_name: 'Noisy', path: 'noisy-icon.png' },
        { display_name: 'Moderate', path: 'moderate-icon.png' },
        { display_name: 'Quiet', path: 'quiet-icon.png' },
        { display_name: 'Silent', path: 'silent-icon.png' }]
    }
  },
  components: {
    Panel,
    Checkbox,
    Icon,
  }
}
</script>

<template>
  <Panel label="Filter">
    <p>Places</p>
    <li v-for="item in items_place">
      <Checkbox :label="item.display_name" />
      <Icon :src="item.path" />
    </li>
    <p>Attributes</p>
    <li v-for="item in items_attributes">
      <Checkbox :label="item.display_name" />
      <Icon :src="item.path" />
    </li>
    <p>Volume</p>
    <li v-for="item in items_volume">
      <Checkbox :label="item.display_name" />
      <Icon :src="item.path" />
    </li>
  </Panel>
</template>


