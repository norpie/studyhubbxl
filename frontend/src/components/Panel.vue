<template>
  <div class="Panel" :class="{ expanded: isExpanded }">
    <div class="header" @click="expandPanel">
      <p class="text">{{ label }}</p>
    </div>
    <div v-if="isExpanded"><slot> </slot></div>
  </div>
</template>

<style scoped>
.Panel {
  opacity: 0.5;
  background-color: #777;
  border-radius: 10px;
  width: 20%;
  text-align: center;
  font-size: larger;
  text-transform: uppercase;
  overflow: hidden;
  transition: max-height 0.5s ease-in-out;
}

.header {
  cursor: pointer;
}

.Panel:hover {
  background-color: #555;
}

.expanded {
  height: 50vh;
  max-height: none;
}

.content {
  margin-top: 10px;
}
</style>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
    props: {
        label:String
        
    },
  data() {
    return {
      isExpanded: false,
    };
  },

  methods: {
    expandPanel(event: MouseEvent) {
      // Vérifie si le clic a été déclenché sur le bouton lui-même
      if ((event.target as HTMLElement).nodeName !== 'BUTTON') {
        this.isExpanded = !this.isExpanded;
      }
    },
  },
});
</script>