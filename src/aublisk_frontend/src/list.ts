import {createApp} from "vue"
import ProjectList from "./components/ProjectList.vue"
import Vue3Toastify from "vue3-toastify";
import "./style.css"
import 'vue3-toastify/dist/index.css';
import Nav from "./components/Nav.vue";
import Footer from "./components/Footer.vue";

createApp(Nav).mount("#nav_header")
createApp(Footer).mount("#footer")

createApp(ProjectList).use(Vue3Toastify, {
    position: "top-center",
    hidePlaceholder: true
}).mount("#project_list")
