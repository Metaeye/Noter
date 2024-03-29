import App from "./App.vue";
import { createApp } from "vue";
import { createPinia } from "pinia";
import { router } from "./router/router";
import ArcoVue from "@arco-design/web-vue";

import "./styles.css";
import "@arco-design/web-vue/dist/arco.css";

const app = createApp(App);
app.use(createPinia());
app.use(router);
app.use(ArcoVue);
app.mount("#app");
