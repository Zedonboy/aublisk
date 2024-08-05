import Dao from "./components/Dao.vue"
import Home from "./components/Dashboard.vue"
import ManageProject from "./components/ManageProject.vue"
import Wallet from "./components/Wallet.vue"
import Profile from "./components/Profile.vue"
import CreateProject from "./components/CreateProject.vue"
import Project from "@/components/Project.vue"
import ViewProfile from "@/components/ViewProfile.vue"
import Proposal from "@/components/Proposal.vue"
const routes = [
    {
        path: "/",
        component: Home
    },

    {
        path: "/manage/project",
        component: ManageProject
    },

    {
        path: "/wallet",
        component: Wallet
    },

    {
        path: "/dao",
        component: Dao
    },

    {
        path: "/profile/create",
        component: Profile
    },

    {
        path: "/profile/read/:id",
        component: ViewProfile
    },

    {
        path: "/project/create",
        component: CreateProject
    },

    {
        path: "/projects/:project_id/proposal/:index",
        component: Proposal
    },

    {
        path: "/project/details/:id",
        component: Project
    }
]
export default routes