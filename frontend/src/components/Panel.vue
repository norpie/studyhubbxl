<template>
    <div class="Panel" :class="{ expanded: isExpanded }">
        <div class="header" @click="expandPanel">
            <p class="text">{{ label }}</p>
        </div>
        <div v-if="isExpanded">
            <slot> </slot>
        </div>
    </div>
</template>

<style scoped>
.Panel {
    opacity: 0.4;
    background-color: #2F86A6;
    border-radius: 10px;
    width: 300px;
    text-align: center;
    font-size: larger;
    text-transform: uppercase;
    border: 1px solid black;
    width: 11%;
    margin: 5px;
    text-align: left;
    font-size: 11px;
    padding-left: 5px;
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
    opacity: 0.6;

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
        label: String

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
