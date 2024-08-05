<template>
    <div class="tw-bg-white">
        <button @click="openPayoutDialog">Open Payout Dialog</button>
        <main
            class="tw-mx-auto tw-px-4 tw-pb-24 tw-pt-14 sm:tw-px-6 sm:tw-pb-32 sm:tw-pt-16 lg:tw-max-w-7xl lg:tw-px-8">
            <!-- Product -->
            <div class="lg:tw-grid lg:tw-grid-cols-8 lg:tw-grid-rows-1 lg:tw-gap-x-8 lg:tw-gap-y-10 xl:tw-gap-x-16">
                <!-- Product details -->
                <div
                    class="tw-mt-14 tw-max-w-2xl sm:tw-mt-16 lg:tw-col-span-4 lg:tw-row-span-2 lg:tw-row-end-2 lg:tw-mt-0 lg:tw-max-w-none">
                    <div class="tw-flex tw-flex-col-reverse">
                        <div class="tw-mt-4">
                            <h1 class="tw-text-2xl tw-font-bold tw-tracking-tight tw-text-gray-900 sm:tw-text-3xl">{{
                                project?.title }}</h1>


                        </div>

                    </div>

                    <div class="tw-mt-10">
                        <button @click="publish" v-if="project_status && 'CREATED' in project_status" type="button"
                            class="tw-flex tw-w-full tw-items-center tw-justify-center tw-rounded-md tw-border tw-border-transparent tw-bg-primary tw-px-8 tw-py-3 tw-text-base tw-font-medium tw-text-white hover:tw-bg-accent focus:tw-outline-none focus:tw-ring-2 focus:tw-ring-indigo-500 focus:tw-ring-offset-2 focus:tw-ring-offset-gray-50">Notify
                            and Publish</button>

                        <button @click="initiate_payout" v-if="project_status && 'INPROGRESS' in project_status" type="button"
                            class="tw-flex tw-w-full tw-items-center tw-justify-center tw-rounded-md tw-border tw-border-transparent tw-bg-primary tw-px-8 tw-py-3 tw-text-base tw-font-medium tw-text-white hover:tw-bg-accent focus:tw-outline-none focus:tw-ring-2 focus:tw-ring-indigo-500 focus:tw-ring-offset-2 focus:tw-ring-offset-gray-50">Initiate
                            Payout</button>

                    </div>
                    <div v-if="!token_info" class="tw-rounded-md tw-bg-blue-50 tw-p-4 tw-mt-4">
                        <div class="tw-flex">
                            <div class="tw-flex-shrink-0">
                                <svg class="tw-h-5 tw-w-5 tw-text-blue-400" viewBox="0 0 20 20" fill="currentColor"
                                    aria-hidden="true">Member
                                    <path fill-rule="evenodd"
                                        d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a.75.75 0 000 1.5h.253a.25.25 0 01.244.304l-.459 2.066A1.75 1.75 0 0010.747 15H11a.75.75 0 000-1.5h-.253a.25.25 0 01-.244-.304l.459-2.066A1.75 1.75 0 009.253 9H9z"
                                        clip-rule="evenodd" />
                                </svg>
                            </div>
                            <div class="tw-ml-3 tw-flex-1 md:tw-flex md:tw-justify-between">
                                <p class="tw-text-sm tw-text-blue-700">{{ function () {
                                    if (!project || !token_info) {
                                        return "Fetching Information"
                                    }

                                    let amount = Number(project?.amount_funded);
                                    let human_amount = amount / 10 ** (token_info.value.decimals)

                                    return `Send ${human_amount}${token_info.value.symbol || token_info.value.name} to
                                    the address bellow`

                                }() }}</p>
                                <p class="tw-mt-3 tw-text-sm md:tw-ml-6 md:tw-mt-0">
                                    <a :href="explorer_link"
                                        class="tw-whitespace-nowrap tw-font-medium tw-text-blue-700 hover:tw-text-blue-600">
                                        Details
                                        <span aria-hidden="true"> &rarr;</span>
                                    </a>
                                </p>
                            </div>
                        </div>
                    </div>
                    <WalletAddress :address="project_wallet" />



                </div>


                <div class="tw-mx-auto tw-mt-16 tw-w-full tw-max-w-2xl lg:tw-col-span-4 lg:tw-mt-0 lg:tw-max-w-none">
                    <TabGroup>
                        <TabList class="tw-flex tw-space-x-1 tw-rounded-xl tw-bg-accent-dark tw-p-1">
                            <Tab as="template" v-slot="{ selected }">
                                <button :class="[
                                    'tw-w-full tw-rounded-lg tw-py-2.5 tw-text-sm tw-font-medium tw-leading-5',
                                    'tw-ring-white/60 tw-ring-offset-2 tw-ring-offset-blue-400 focus:tw-outline-none focus:tw-ring-2',
                                    selected ? 'tw-bg-white tw-text-accent-dark tw-shadow' : 'tw-text-blue-100 hover:tw-bg-white/[0.12] hover:tw-text-white',
                                ]">
                                    Proposals {{ proposals_ref?.length || 0 }}
                                </button>
                            </Tab>
                            <Tab as="template" v-slot="{ selected }">
                                <button :class="[
                                    'tw-w-full tw-rounded-lg tw-py-2.5 tw-text-sm tw-font-medium tw-leading-5',
                                    'tw-ring-white/60 tw-ring-offset-2 tw-ring-offset-blue-400 focus:tw-outline-none focus:tw-ring-2',
                                    selected ? 'tw-bg-white tw-text-accent-dark tw-shadow' : 'tw-text-blue-100 hover:tw-bg-white/[0.12] hover:tw-text-white',
                                ]">
                                    Milestone {{ project?.milestones.length || 0 }}
                                </button>
                            </Tab>
                        </TabList>
                        <TabPanels>
                            <TabPanel :class="[
                                'tw-rounded-xl tw-bg-white tw-px-3',
                                'tw-ring-white/60 tw-ring-offset-2 tw-ring-offset-blue-400 focus:tw-outline-none focus:tw-ring-2 tw-mt-4 lg:h-[40vh]',
                            ]">
                                <ul>
                                    <div v-for="(proposal, idx) in proposals_ref" :key="idx"
                                        class="tw-flex tw-space-x-4 tw-text-sm tw-text-gray-500">
                                        <div class="tw-flex-none tw-py-4">
                                            <!-- <img :src="review.avatarSrc" alt=""
                                                class="tw-h-10 tw-w-10 tw-rounded-full tw-bg-gray-100" /> -->
                                        </div>
                                        <div
                                            :class="[idx === 0 ? '' : 'tw-border-t tw-border-gray-200', 'tw-flex-1 tw-py-4']">
                                            <a class="tw-font-medium tw-text-gray-900 tw-text-ellipsis">{{
                                                proposal.sender.toText() }}</a>
                                            <p>
                                                <time class="tw-text-xs"
                                                    :datetime="new Date(Number(proposal.timestamp).toString())">{{
                                                        formatDistanceToNow(new Date(Number(proposal.timestamp))) }}</time>
                                            </p>


                                            <div class="tw-prose tw-prose-sm tw-mt-1 tw-max-w-none tw-text-gray-500"
                                                v-html="clipString(proposal.content)" />
                                            <div class=" tw-grid tw-grid-cols-2 tw-gap-2">
                                                <button @click="assign_member(proposal.sender)"
                                                    class="tw-flex tw-w-full tw-col-span-1 tw-items-center tw-justify-center tw-rounded-md tw-border tw-border-transparent tw-bg-accent text-px-2 tw-py-1.5 tw-text-sm tw-font-medium tw-text-white hover:tw-bg-accent-dark focus:tw-outline-none focus:tw-ring-2 focus:tw-ring-indigo-500 focus:tw-ring-offset-2 focus:tw-ring-offset-gray-50">Assign</button>
                                                <button type="button"
                                                    class="tw-flex tw-w-full tw-col-span-1 tw-items-center tw-justify-center tw-rounded-md tw-border-transparent tw-border-accent tw-border-2 text-px-2 tw-py-1.5 tw-text-sm tw-font-medium tw-text-accent hover:tw-border-accent-dark hover:tw-text-accent-dark focus:tw-outline-none focus:tw-ring-2 focus:tw-ring-indigo-500 focus:tw-ring-offset-2 focus:tw-ring-offset-gray-50">Detail</button>
                                            </div>
                                        </div>

                                    </div>

                                </ul>
                            </TabPanel>
                            <TabPanel>
                                <li v-for="post in categories.Milestones" :key="post.id"
                                    class="tw-relative tw-rounded-md tw-p-3 hover:tw-bg-gray-100">
                                    <h3 class="tw-text-sm tw-font-medium tw-leading-5">
                                        {{ post.title }}
                                    </h3>
                                    <ul
                                        class="tw-mt-1 tw-flex tw-space-x-1 tw-text-xs tw-font-normal tw-leading-4 tw-text-gray-500">
                                        <li>{{ post.date }}</li>
                                        <li>&middot;</li>
                                        <li>{{ post.commentCount }} comments</li>
                                        <li>&middot;</li>
                                        <li>{{ post.shareCount }} shares</li>
                                    </ul>

                                    <a href="#" :class="[
                                        'tw-absolute tw-inset-0 tw-rounded-md',
                                        'tw-ring-blue-400 focus:tw-z-10 focus:tw-outline-none focus:tw-ring-2',
                                    ]" />
                                </li>
                                <button type="button"
                                    class="tw-flex tw-w-full tw-col-span-1 tw-items-center tw-justify-center tw-rounded-md tw-border-transparent tw-border-accent tw-border-2 text-px-2 tw-py-1.5 tw-text-sm tw-font-medium tw-text-accent hover:tw-text-white hover:tw-bg-accent focus:tw-outline-none focus:tw-ring-2 focus:tw-ring-indigo-500 focus:tw-ring-offset-2 focus:tw-ring-offset-gray-50">Add
                                    Milestone</button>
                            </TabPanel>
                        </TabPanels>
                    </TabGroup>

                </div>

            </div>

            <div class="tw-mt-6 tw-text-gray-500" v-html="project?.description || ''"></div>
        </main>


    </div>
</template>
<script setup lang="ts">
import { onMounted, ref } from 'vue'
import {
    Tab,
    TabGroup,
    TabList,
    TabPanel,
    TabPanels
} from '@headlessui/vue'

import WalletAddress from "@/widgets/WalletAddress.vue"
import { useRoute } from "vue-router"
import { toast } from 'vue3-toastify'
import { createActor } from '../../../declarations/aublisk_backend'
import { APP_CANISTER_ID } from '@/config'
import { ProjectStatus, Proposal } from '../../../declarations/aublisk_backend/aublisk_backend.did'
import { Project } from '../../../declarations/aublisk_backend/aublisk_backend.did'
import { TokenInfo } from '@/utils/types'
import { clipString, getTokenInfo } from '@/utils'
import { encodeIcrcAccount, IcrcAccount } from "@dfinity/ledger-icrc"
import { Principal } from '@dfinity/principal'
import { hexStringToUint8Array } from '@dfinity/utils'
import { formatDistanceToNow } from "date-fns"
import { usePayoutDialog } from "@/plugins/payoutDialog"
import { useDialog } from "@/plugins/simpleDialog"
// Testing
const openPayoutDialog = async () => {

    usePayoutDialog().showPayoutDialog({
        emails: ["fsdfdsfds", "fsfnlndfsl"],
        onConfirm(data) {
            console.log(data)
        },
        //@ts-ignore
        token_symbol: token_info.value?.symbol
    })

}

let project_status = ref<ProjectStatus | null>(null);
let project = ref<Project | null>(null);
let proposals_ref = ref<Proposal[] | null>();
let token_info = ref<TokenInfo | null>(null)
let project_wallet = ref("")
let explorer_link = ref("#")
let workers = ref<Principal[]>([])
onMounted(() => {
    let router = useRoute()
    let project_hash = router.params.id
    if (!window.identity) {
        return toast.error("You are not authorized")
    }

    let actor = createActor(APP_CANISTER_ID, {
        agentOptions: {
            identity: window.identity
        }
    })

    let all_calls = Promise.all([actor.read_project_and_status(project_hash), actor.get_project_proposals(project_hash), actor.get_project_workers(project_hash)])
    toast.promise(all_calls, { pending: "Making Blockchain Calls", error: "Error during blockchain call", success: "Success" }).then(data => {
        let [proj] = data[0]
        let result = data[1]
        let result_2 = data[2]
        if (proj) {
            let prjkt = proj[0]
            let p_status = proj[1]
            getTokenInfo(prjkt.icrc_token_address).then(data => {
                token_info.value = data
            })
            project_status.value = p_status;
            project.value = prjkt;
            let project_account: IcrcAccount = {
                owner: Principal.fromText(APP_CANISTER_ID),
                subaccount: hexStringToUint8Array(prjkt.hash_id)
            }

            let p_wallet = encodeIcrcAccount(project_account)

            project_wallet.value = p_wallet
            explorer_link.value = `https://dashboard.internetcomputer.org/account/${p_wallet}`
        }

        if ("Ok" in result) {
            let proposals = result.Ok
            proposals_ref.value = proposals
        } else if ("Err" in result) {
            toast.error(result.Err)
        }
        //@ts-ignore
        workers.value = result_2


    })
})


function assign_member(member: Principal) {
    if (!window.identity) {
        return toast.error("You are not authorized")
    }

    let actor = createActor(APP_CANISTER_ID, {
        agentOptions: {
            identity: window.identity
        }
    })

    let project_hash = router.params.id

    let action = actor.assign_member_to_milestone(project_hash, member, -1)
    toast.promise(action, {
        "pending": "Calling Blockchain",
        "error": "Error during Blockchain call",
        success: "Success"
    }).then(data => {
        if ("Err" in data) {
            toast.error(data.Err)
        } else if ("Ok" in data) {
            toast.success("User Assigned Successfully")
        }
    })
}

function initiate_payout() {
    usePayoutDialog().showPayoutDialog({
        emails: workers.value.map(p => {
            return p.toText() //clipString(p.toText(), 9)
        }),
        onConfirm(data) {
            let router = useRoute()
            let project_hash = router.params.id
            if (!window.identity) {
                return toast.error("You are not authorized")
            }

            let actor = createActor(APP_CANISTER_ID, {
                agentOptions: {
                    identity: window.identity
                }
            })

            toast.promise(actor.manual_payout(project_hash, [[Principal.fromHex(data.email), BigInt(data.amount)]]), {
                pending: "Sending Payment to User",
                error: "Error Sending Payment",
                success: "Success"
            }).then(data => {
                if("Err" in data) {
                    toast.error(data.Err)
                }
            })
        },
        //@ts-ignore
        token_symbol: token_info.value?.symbol
    })
}

function publish() {
    let router = useRoute()
    let project_hash = router.params.id
    if (!window.identity) {
        return toast.error("You are not authorized")
    }

    let showdialog = useDialog().showDialog;

    showdialog({
        title: "Confirmation",
        message: "You about to get your funded project published, you will receive proposals from members",
        cancelText: "Cancel",
        confirmText: "Ok",
        onConfirm() {
            let actor = createActor(APP_CANISTER_ID, {
                agentOptions: {
                    identity: window.identity
                }
            })

            let action = actor.notify_project_fund(project_hash as string)
            toast.promise(action, {
                pending: "Calling Blockchain",
                error: "Error Calling Blockchain",
                success: "Success"
            }).then(data => {
                if ("Err" in data) {
                    toast.error(data.Err)
                }


            })
        },
        onCancel() {

        },
    })



}
const categories = ref({
    Proposals: [
        {
            id: 1,
            title: 'Does drinking coffee make you smarter?',
            date: '5h ago',
            commentCount: 5,
            shareCount: 2,
        },
        {
            id: 2,
            title: "So you've bought coffee... now what?",
            date: '2h ago',
            commentCount: 3,
            shareCount: 2,
        },
    ],
    Milestones: [
        {
            id: 1,
            title: 'Is tech making coffee better or worse?',
            date: 'Jan 7',
            commentCount: 29,
            shareCount: 16,
        },
        {
            id: 2,
            title: 'The most innovative things happening in coffee',
            date: 'Mar 19',
            commentCount: 24,
            shareCount: 12,
        },
    ]
})


const open = ref(false)
</script>