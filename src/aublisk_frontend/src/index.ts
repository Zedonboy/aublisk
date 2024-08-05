import {createApp} from "vue"
import Hello from "./components/Hello.vue"
import LatestProjects from "./components/LatestProjects.vue"
import "./style.css"
import Nav from "./components/Nav.vue"
import Footer from "./components/Footer.vue"
import Vue3Toastify, { ToastContainerOptions } from "vue3-toastify"
import 'vue3-toastify/dist/index.css';
createApp(Nav).mount("#nav_header")
createApp(Footer).mount("#footer")

createApp(LatestProjects).use(Vue3Toastify, {
    position: "top-center",
    hideProgressBar: true
    } as ToastContainerOptions).mount("#project_list")
