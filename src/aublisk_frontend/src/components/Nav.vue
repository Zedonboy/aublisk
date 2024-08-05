<template>
    <!-- Ace Responsive Menu -->
    <nav class="posr">
        <div class="container posr menu_bdrt1">
            <div class="row align-items-center justify-content-between">
                <div class="col-auto px-0">
                    <div class="d-flex align-items-center justify-content-between">
                        <div class="logos">
                            <a class="header-logo logo2" href="/"><img class="tw-w-36" src="@/static/images/aublisk_web.png"
                                    alt="Header Logo"></a>
                        </div>
                        <div class="home1_style">

                        </div>
                        <!-- Responsive Menu Structure-->
                        <ul id="respMenu" class="ace-responsive-menu tw-ml-8" data-menu-style="horizontal">
                            <li class="visible_list"> <a class="list-item" href="/"><span class="title">Home</span></a>
                            </li>

                            <li> <a class="list-item" href="/list">Projects</a></li>
                        </ul>
                    </div>
                </div>
                <div class="col-auto px-0">
                    <div class="d-flex align-items-center">
                        <a v-if="auth" class="ud-btn btn-thm add-joining">Go to Dashboard</a>
                        <button @click="sign_in" v-else class="ud-btn btn-thm add-joining">Sign With Internet Identity</button>
                    </div>
                </div>
            </div>
        </div>
    </nav>
</template>
<script setup lang="ts">
import { AuthClient } from "@dfinity/auth-client"
import { onMounted, ref } from "vue";
import { IDENTITY_PROVIDER } from "../config";

let auth = ref(false)
onMounted(() => {
    let task = async () => {
        let client = await AuthClient.create()
        let is_auth = await client.isAuthenticated()
        auth.value = is_auth
    }

    task()
})
async function sign_in() {
    let client = await AuthClient.create()
    client.login({
        identityProvider: IDENTITY_PROVIDER,
        derivationOrigin: "https://aublisk.com",
        onSuccess() {
            let identity = client.getIdentity()
            window.identity = identity
            auth.value = true
        }
    })
}
</script>