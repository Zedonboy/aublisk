<template>
    <div v-for="(project, idx) in list" class="">
        <div class="freelancer-style1 bdrs12 bdr1 hover-box-shadow">
            <div class="d-flex align-items-center">
                <div class="thumb w60 position-relative rounded-circle mr15">
                    <img v-if="token[project.icrc_token_address]" class="rounded-circle mx-auto" :src="token[project.icrc_token_address].logo_url" alt="">
                    <!-- <span class="online-badge2"></span> -->
                </div>
                <div>
                    <span class="mb-0 fz14 dark-color list-inline-item mb5-sm"><i
                            class="flaticon-place fz12 me-1"></i>{{ clipString(project.owner.toText(), 6) }}</span>
                    <span class="mb-0 fz14 dark-color list-inline-item mb5-sm"><i
                            class="flaticon-30-days fz12 me-1"></i> {{ formatDistance(new
                                Date(Number(project.timestamp)), new Date()) }}</span>
                    <!-- <span class="mb-0 fz14 dark-color list-inline-item mb5-sm"><i
                            class="flaticon-contract fz12 me-1"></i> 1 Received</span> -->
                </div>
            </div>
            <div class="details mt-4">
                <h5 class="title mb-3">{{ project.title }}</h5>
                <div class="text mt10 line-clamp2" v-html="clipString(project.description, 50)"></div>
                <div class="skill-tags d-flex align-items-center mb20">
                    <span v-for="skill in project.skill_sets" class="tag">{{ skill }}</span>

                </div>
                <h4><small class="text fz14 me-3">Budget</small>{{ wallet_balance[idx] }} {{ token[project.icrc_token_address]?.symbol || "Token" }}</h4>
                <div class="d-grid mt15">
                    <a :href="`/project?project_id=${project.hash_id}`" class="ud-btn btn-thm-border bdrs12 default-box-shadow1">Send
                        Proposal<i class="fal fa-arrow-right-long"></i></a>
                </div>
            </div>
        </div>
    </div>
</template>
<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { Project } from '../../../declarations/aublisk_backend/aublisk_backend.did';
import { createActor } from "../../../declarations/aublisk_backend"
import { clipString, getTokenInfo } from '../utils';
import { formatDistance } from 'date-fns';
import { APP_CANISTER_ID } from '../config';
import { toast } from 'vue3-toastify';
import { TokenInfo } from '../utils/types';

type TokenCache = {
  [key: string]: TokenInfo;
};
let list = ref<Project[]>([])
let wallet_balance : number[] = []
let token : TokenCache = {}
onMounted(() => {
    try {
        
    } catch (error) {
        
    }
    let actor = createActor(APP_CANISTER_ID)
    let action = actor.get_latest_projects()
    toast.promise(action, {
        pending: "Fetching Gigs",
        error: "Error fetching Gig",
        success: "Success"
    }).then(data => {

        let task = async () => {
            for (let index = 0; index < data.length; index++) {
                const project = data[index];
                let info = token[project.icrc_token_address]
                if (info) {
                    let human_amount = Number(project.amount_funded) / 10 ** (info.decimals)
                    wallet_balance.push(human_amount)
                    continue
                } else {
                    let info = await getTokenInfo(project.icrc_token_address)
                    let human_amount = Number(project.amount_funded) / 10 ** (info.decimals)
                    wallet_balance.push(human_amount)
                    token[project.icrc_token_address] = info;
                }

            }

        }

        task().finally(() => {
            // fetch balance
            list.value = data.slice(0, 8)
        })



    })
})
</script>