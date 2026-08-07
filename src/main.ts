import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import router from "./presentation/router";
import { useThemeStore } from "./presentation/stores/themeStore";

const app = createApp(App);
const pinia = createPinia();

app.use(pinia);
app.use(router);

useThemeStore();

app.mount("#app");
