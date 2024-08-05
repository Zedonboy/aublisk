import { createApp } from "vue";
import Project from "./components/Project.vue";
import Vue3Toastify, { ToastContainerOptions } from "vue3-toastify";
import "./style.css"
import 'vue3-toastify/dist/index.css';
import Nav from "./components/Nav.vue";
import Footer from "./components/Footer.vue";
createApp(Nav).mount("#nav_header")
createApp(Footer).mount("#footer")

createApp(Project).use(Vue3Toastify, {
    position: "top-center",
    hideProgressBar: true
} as ToastContainerOptions).mount("#project_page")