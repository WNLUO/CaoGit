import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import "./stores/themeStore"; // Initialize theme

createApp(App).mount("#app");
