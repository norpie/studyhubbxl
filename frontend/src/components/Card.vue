<template>
    <div class="card">
        <div class="card-rectangle">{{ label }}
            <ClickableIcon>
                <Icon class="star" src="star-empty.png" />
            </ClickableIcon>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import Icon from "./Icon.vue";
import ClickableIcon from "./ClickableIcon.vue";

export default defineComponent({
    props: {
        label: String,
    },
    components: { Icon, ClickableIcon }
});
</script>

<style scoped>
.card {
    display: flex;
    justify-content: flex-end;
    align-items: flex-end;
    width: 90%;
    height: 100px;
    margin: 10px;
}

.card-rectangle {
    width: 100%;
    height: 100%;
    background-color: lightgray;
    color: black;
    font-weight: bold;
}

.star {
    height: 25px;
    width: 25px;
    transform: translateY(60px);
}

.clickable-icon{

    float: right;
}
</style>
