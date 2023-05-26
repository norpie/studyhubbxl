import { createApp } from "vue";
import App from "./App.vue";

import VueMapboxTs from "vue-mapbox-ts";

const app = createApp(App);

app.use(VueMapboxTs);
app.mount("#app");
