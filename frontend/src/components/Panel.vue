<template>
    <div class="Panel" :class="{ expanded: isExpanded }">
        <div class="header" @click="expandPanel">
            <p class="text">{{ label }}</p>
        </div>
        <div v-if="isExpanded">
            <slot></slot>
        </div>
    </div>
</template>

<style scoped>
.Panel {
    opacity: 0.7;
    background-color: #34BE82 ;
    border-radius: 10px;
    width: 220px;
    text-align: center;
    font-size: larger;
    text-transform: uppercase;
    border: 1px solid black;
    margin: 5px 5px 5px 5px;
    text-align: left;
    font-size: 11px;
    padding: 5px 5px 5px 5px;
    list-style-type: none;
    overflow: hidden;
    transition: max-height 0.5s ease-in-out;
}


.text {
    text-align: center;
}

.header {
    cursor: pointer;
}

.Panel:hover {
    opacity: 0.9;

}

.expanded {
    height: 65vh;
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
        label: String

    },
    data() {
        return {
            isExpanded: false,
        };
    },

    methods: {
        expandPanel(event: MouseEvent) {
            if ((event.target as HTMLElement).nodeName !== 'BUTTON') {
                this.isExpanded = !this.isExpanded;
            }
        },
        toggle() {
            this.isExpanded = !this.isExpanded;
        }
    },
});
</script>
