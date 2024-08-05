import { createApp } from "vue";
import { createWebHistory, createRouter } from "vue-router";
import routes from "./routes";
import "./style.css";
import { QRModalPlugin } from "./plugins/qrcode";
import { SendFundsPlugin } from "./plugins/sendFunds";
import DialogPlugin from "./plugins/simpleDialog"
import PayoutDialog from "./plugins/payoutDialog"
import Vue3Toasity, { type ToastContainerOptions } from 'vue3-toastify';
import 'vue3-toastify/dist/index.css';
let router = createRouter({
  history: createWebHistory(),
  routes,
});

window.global ||= window;

import App from "./App.vue";
import Nav from "./components/Nav.vue";
import MobileNav from "./components/MobileNav.vue";

createApp(Nav).use(router).mount("#app_side_nav");
createApp(MobileNav).use(router).mount("#app_side_nav_mobile");
createApp(App).use(router).use(QRModalPlugin).use(SendFundsPlugin).use(Vue3Toasity, {
position: "top-center",
hideProgressBar: true
} as ToastContainerOptions).use(DialogPlugin).use(PayoutDialog).mount("#app");
