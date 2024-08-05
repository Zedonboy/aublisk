<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { createActor } from "../../../declarations/aublisk_backend"
import MobileNav from './MobileNav.vue';
import { AuthClient } from "@dfinity/auth-client"
import { APP_CANISTER_ID } from '@/config';
import { toast } from 'vue3-toastify';
import { Project, ProjectStatus } from '../../../declarations/aublisk_backend/aublisk_backend.did';
import { get_icrc_info } from '@/utils';
import { IcrcTokenMetadataResponse } from '@dfinity/ledger-icrc';

const is_loading = ref(false)
const projects_state = ref<[Project, ProjectStatus, IcrcTokenMetadataResponse][] | null>(null)
onMounted(() => {
   
    let task = async () => {
        let client = await AuthClient.create()
        if (! await client.isAuthenticated()) {
            toast.error("You are not authenticated")
            return
        }

        let actor = createActor(APP_CANISTER_ID, {
            agentOptions: {
                //@ts-ignore
                identity: client.getIdentity()
            }
        })

        let projects = await actor.get_user_projects();

        let cache = {}
        let transformed_data = []
        for (const data of projects) {
            //@ts-ignore
            if (!cache[data[0].icrc_token_address]) {
                let coinInfo = await get_icrc_info(data[0].icrc_token_address);
                //@ts-ignore
                cache[data[0].icrc_token_address] = coinInfo;
                let n_d = [...data, coinInfo]
                transformed_data.push(n_d)
            } else continue

        }

        //@ts-ignore
        projects_state.value = transformed_data
    }

    is_loading.value = true

    task().finally(() => {
        is_loading.value = false
    })
})
</script>

<template>
    <div class="row pb40">
        <div class="col-lg-12">
            <div class="dashboard_navigationbar d-block d-lg-none">
                <MobileNav />
            </div>
        </div>
        <div class="col-lg-9">
            <div class="dashboard_title_area">
                <h2>Manage Project</h2>
                <!-- <p class="text"></p> -->
            </div>
        </div>
        <div class="col-lg-3">
            <div class="text-lg-end">
                <RouterLink to="/project/create" class="ud-btn btn-dark default-box-shadow2">
                    Create
                    Project<i class="fal fa-arrow-right-long"></i>
                </RouterLink>
                <!-- <a href="page-dashboard-create-project.html" class="ud-btn btn-dark default-box-shadow2"></a> -->
            </div>
        </div>
    </div>
    <div class="row">
        <div class="col-xl-12">
            <div class="ps-widget bgc-white bdrs4 p30 mb30 overflow-hidden position-relative">
                <div class="navtab-style1">
                    <nav>
                        <div class="nav nav-tabs mb30" id="nav-tab2" role="tablist">
                            <button class="nav-link active fw500 ps-0" id="nav-item1-tab" data-bs-toggle="tab"
                                data-bs-target="#nav-item1" type="button" role="tab" aria-controls="nav-item1"
                                aria-selected="true">All Projects</button>
                            <!-- <button class="nav-link fw500" id="nav-item2-tab" data-bs-toggle="tab"
                                data-bs-target="#nav-item2" type="button" role="tab" aria-controls="nav-item2"
                                aria-selected="false">Pending Projects</button>
                            <button class="nav-link fw500" id="nav-item3-tab" data-bs-toggle="tab"
                                data-bs-target="#nav-item3" type="button" role="tab" aria-controls="nav-item3"
                                aria-selected="false">Ongoing Projects</button>
                            <button class="nav-link fw500" id="nav-item4-tab" data-bs-toggle="tab"
                                data-bs-target="#nav-item4" type="button" role="tab" aria-controls="nav-item4"
                                aria-selected="false">Expired Projects</button>
                            <button class="nav-link fw500" id="nav-item5-tab" data-bs-toggle="tab"
                                data-bs-target="#nav-item5" type="button" role="tab" aria-controls="nav-item5"
                                aria-selected="false">Completed Projects</button>
                            <button class="nav-link fw500" id="nav-item6-tab" data-bs-toggle="tab"
                                data-bs-target="#nav-item6" type="button" role="tab" aria-controls="nav-item6"
                                aria-selected="false">Canceled Projects</button> -->
                        </div>
                    </nav>
                    <div class="tab-content" id="nav-tabContent">
                        <div class="tab-pane fade show active" id="nav-item1" role="tabpanel"
                            aria-labelledby="nav-item1-tab">
                            <div class="packages_table table-responsive">
                                <table class="table-style3 table at-savesearch">
                                    <thead class="t-head">
                                        <tr>
                                            <th scope="col">Title</th>
                                            <th scope="col">Status</th>
                                            <th scope="col">Budget</th>
                                            <!-- <th scope="col">Actions</th> -->
                                        </tr>
                                    </thead>
                                    <tbody class="t-body">

                                        <tr v-for="item in projects_state">
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">{{ item[0].title }}</h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i></p>
                                                            <!-- <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p> -->
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">{{
                                                                Object.keys(item[1])[0] }}</span></td>
                                            <td class="vam"><span class="fz14 fw400">{{ function(){
                                                let logo
                                                let symbol
                                                let decimals = 8
                                                for (const data of item[2]) {
                                                    if (data[0] == "icrc1:decimals") {
                                                        //@ts-ignore
                                                        let d = data[1].Nat
                                                        decimals = Number(d)
                                                    }

                                                    if (data[0] == "icrc1:symbol") {
                                                        //@ts-ignore
                                                        let d = data[1].Text
                                                        symbol = d
                                                    }

                                                    if(data[0] == "icrc1:logo") {
                                                        //@ts-ignore
                                                        let d = data[1].Text
                                                        logo = d
                                                    }
                                                }

                                                let amount = item[0].amount_funded / BigInt(Math.pow(10, decimals))
                                                return `${Number(amount)} ${symbol}`
                                            }() }}</span></td>
                                            <!-- <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td> -->
                                        </tr>

                                        
                                       
<!--                                        
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">Food Delviery Mobile App</h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr> -->
                                    </tbody>
                                </table>
                                <!-- <div class="mbp_pagination text-center mt30">
                                    <ul class="page_navigation">
                                        <li class="page-item">
                                            <a class="page-link" href="#"> <span class="fas fa-angle-left"></span></a>
                                        </li>
                                        <li class="page-item"><a class="page-link" href="#">1</a></li>
                                        <li class="page-item active" aria-current="page">
                                            <a class="page-link" href="#">2 <span class="sr-only">(current)</span></a>
                                        </li>
                                        <li class="page-item"><a class="page-link" href="#">3</a></li>
                                        <li class="page-item"><a class="page-link" href="#">4</a></li>
                                        <li class="page-item"><a class="page-link" href="#">5</a></li>
                                        <li class="page-item"><a class="page-link" href="#">...</a></li>
                                        <li class="page-item"><a class="page-link" href="#">20</a></li>
                                        <li class="page-item">
                                            <a class="page-link" href="#"><span class="fas fa-angle-right"></span></a>
                                        </li>
                                    </ul>
                                    <p class="mt10 mb-0 pagination_page_count text-center">1 – 20 of 300+ property
                                        available</p>
                                </div> -->
                            </div>
                        </div>
                        <div class="tab-pane fade" id="nav-item2" role="tabpanel" aria-labelledby="nav-item2-tab">
                            <div class="packages_table table-responsive">
                                <table class="table-style3 table at-savesearch">
                                    <thead class="t-head">
                                        <tr>
                                            <th scope="col">Title</th>
                                            <th scope="col">Category</th>
                                            <th scope="col">Type/Cost</th>
                                            <th scope="col">Actions</th>
                                        </tr>
                                    </thead>
                                    <tbody class="t-body">
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">Food Delviery Mobile App</h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">Swift / SwiftUI Developer for B2B iOS
                                                                apps</h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">English content writer for Fintech
                                                            </h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">Full-stack Developer to help us to
                                                                build our</h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">Looking for team members for web
                                                                agency</h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">Video animator to bring some
                                                                illustrations to life</h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">Food Delviery Mobile App</h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                    </tbody>
                                </table>
                                <div class="mbp_pagination text-center mt30">
                                    <ul class="page_navigation">
                                        <li class="page-item">
                                            <a class="page-link" href="#"> <span class="fas fa-angle-left"></span></a>
                                        </li>
                                        <li class="page-item"><a class="page-link" href="#">1</a></li>
                                        <li class="page-item active" aria-current="page">
                                            <a class="page-link" href="#">2 <span class="sr-only">(current)</span></a>
                                        </li>
                                        <li class="page-item"><a class="page-link" href="#">3</a></li>
                                        <li class="page-item"><a class="page-link" href="#">4</a></li>
                                        <li class="page-item"><a class="page-link" href="#">5</a></li>
                                        <li class="page-item"><a class="page-link" href="#">...</a></li>
                                        <li class="page-item"><a class="page-link" href="#">20</a></li>
                                        <li class="page-item">
                                            <a class="page-link" href="#"><span class="fas fa-angle-right"></span></a>
                                        </li>
                                    </ul>
                                    <p class="mt10 mb-0 pagination_page_count text-center">1 – 20 of 300+ property
                                        available</p>
                                </div>
                            </div>
                        </div>
                        <div class="tab-pane fade" id="nav-item3" role="tabpanel" aria-labelledby="nav-item3-tab">
                            <div class="packages_table table-responsive">
                                <table class="table-style3 table at-savesearch">
                                    <thead class="t-head">
                                        <tr>
                                            <th scope="col">Title</th>
                                            <th scope="col">Category</th>
                                            <th scope="col">Type/Cost</th>
                                            <th scope="col">Actions</th>
                                        </tr>
                                    </thead>
                                    <tbody class="t-body">
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">Food Delviery Mobile App</h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">Swift / SwiftUI Developer for B2B iOS
                                                                apps</h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">English content writer for Fintech
                                                            </h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">Full-stack Developer to help us to
                                                                build our</h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">Looking for team members for web
                                                                agency</h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">Video animator to bring some
                                                                illustrations to life</h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">Food Delviery Mobile App</h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                    </tbody>
                                </table>
                                <div class="mbp_pagination text-center mt30">
                                    <ul class="page_navigation">
                                        <li class="page-item">
                                            <a class="page-link" href="#"> <span class="fas fa-angle-left"></span></a>
                                        </li>
                                        <li class="page-item"><a class="page-link" href="#">1</a></li>
                                        <li class="page-item active" aria-current="page">
                                            <a class="page-link" href="#">2 <span class="sr-only">(current)</span></a>
                                        </li>
                                        <li class="page-item"><a class="page-link" href="#">3</a></li>
                                        <li class="page-item"><a class="page-link" href="#">4</a></li>
                                        <li class="page-item"><a class="page-link" href="#">5</a></li>
                                        <li class="page-item"><a class="page-link" href="#">...</a></li>
                                        <li class="page-item"><a class="page-link" href="#">20</a></li>
                                        <li class="page-item">
                                            <a class="page-link" href="#"><span class="fas fa-angle-right"></span></a>
                                        </li>
                                    </ul>
                                    <p class="mt10 mb-0 pagination_page_count text-center">1 – 20 of 300+ property
                                        available</p>
                                </div>
                            </div>
                        </div>
                        <div class="tab-pane fade" id="nav-item4" role="tabpanel" aria-labelledby="nav-item4-tab">
                            <div class="packages_table table-responsive">
                                <table class="table-style3 table at-savesearch">
                                    <thead class="t-head">
                                        <tr>
                                            <th scope="col">Title</th>
                                            <th scope="col">Category</th>
                                            <th scope="col">Type/Cost</th>
                                            <th scope="col">Actions</th>
                                        </tr>
                                    </thead>
                                    <tbody class="t-body">
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">Food Delviery Mobile App</h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">Swift / SwiftUI Developer for B2B iOS
                                                                apps</h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">English content writer for Fintech
                                                            </h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">Full-stack Developer to help us to
                                                                build our</h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">Looking for team members for web
                                                                agency</h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">Video animator to bring some
                                                                illustrations to life</h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">Food Delviery Mobile App</h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                    </tbody>
                                </table>
                                <div class="mbp_pagination text-center mt30">
                                    <ul class="page_navigation">
                                        <li class="page-item">
                                            <a class="page-link" href="#"> <span class="fas fa-angle-left"></span></a>
                                        </li>
                                        <li class="page-item"><a class="page-link" href="#">1</a></li>
                                        <li class="page-item active" aria-current="page">
                                            <a class="page-link" href="#">2 <span class="sr-only">(current)</span></a>
                                        </li>
                                        <li class="page-item"><a class="page-link" href="#">3</a></li>
                                        <li class="page-item"><a class="page-link" href="#">4</a></li>
                                        <li class="page-item"><a class="page-link" href="#">5</a></li>
                                        <li class="page-item"><a class="page-link" href="#">...</a></li>
                                        <li class="page-item"><a class="page-link" href="#">20</a></li>
                                        <li class="page-item">
                                            <a class="page-link" href="#"><span class="fas fa-angle-right"></span></a>
                                        </li>
                                    </ul>
                                    <p class="mt10 mb-0 pagination_page_count text-center">1 – 20 of 300+ property
                                        available</p>
                                </div>
                            </div>
                        </div>
                        <div class="tab-pane fade" id="nav-item5" role="tabpanel" aria-labelledby="nav-item5-tab">
                            <div class="packages_table table-responsive">
                                <table class="table-style3 table at-savesearch">
                                    <thead class="t-head">
                                        <tr>
                                            <th scope="col">Title</th>
                                            <th scope="col">Category</th>
                                            <th scope="col">Type/Cost</th>
                                            <th scope="col">Actions</th>
                                        </tr>
                                    </thead>
                                    <tbody class="t-body">
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">Food Delviery Mobile App</h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">Swift / SwiftUI Developer for B2B iOS
                                                                apps</h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">English content writer for Fintech
                                                            </h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">Full-stack Developer to help us to
                                                                build our</h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">Looking for team members for web
                                                                agency</h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">Video animator to bring some
                                                                illustrations to life</h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">Food Delviery Mobile App</h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                    </tbody>
                                </table>
                                <div class="mbp_pagination text-center mt30">
                                    <ul class="page_navigation">
                                        <li class="page-item">
                                            <a class="page-link" href="#"> <span class="fas fa-angle-left"></span></a>
                                        </li>
                                        <li class="page-item"><a class="page-link" href="#">1</a></li>
                                        <li class="page-item active" aria-current="page">
                                            <a class="page-link" href="#">2 <span class="sr-only">(current)</span></a>
                                        </li>
                                        <li class="page-item"><a class="page-link" href="#">3</a></li>
                                        <li class="page-item"><a class="page-link" href="#">4</a></li>
                                        <li class="page-item"><a class="page-link" href="#">5</a></li>
                                        <li class="page-item"><a class="page-link" href="#">...</a></li>
                                        <li class="page-item"><a class="page-link" href="#">20</a></li>
                                        <li class="page-item">
                                            <a class="page-link" href="#"><span class="fas fa-angle-right"></span></a>
                                        </li>
                                    </ul>
                                    <p class="mt10 mb-0 pagination_page_count text-center">1 – 20 of 300+ property
                                        available</p>
                                </div>
                            </div>
                        </div>
                        <div class="tab-pane fade" id="nav-item6" role="tabpanel" aria-labelledby="nav-item6-tab">
                            <div class="packages_table table-responsive">
                                <table class="table-style3 table at-savesearch">
                                    <thead class="t-head">
                                        <tr>
                                            <th scope="col">Title</th>
                                            <th scope="col">Category</th>
                                            <th scope="col">Type/Cost</th>
                                            <th scope="col">Actions</th>
                                        </tr>
                                    </thead>
                                    <tbody class="t-body">
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">Food Delviery Mobile App</h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">Swift / SwiftUI Developer for B2B iOS
                                                                apps</h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">English content writer for Fintech
                                                            </h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">Full-stack Developer to help us to
                                                                build our</h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">Looking for team members for web
                                                                agency</h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">Video animator to bring some
                                                                illustrations to life</h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <th scope="row">
                                                <div
                                                    class="freelancer-style1 box-shadow-none row m-0 p-0 align-items-lg-end">
                                                    <div class="d-lg-flex px-0">
                                                        <div class="details mb15-md-md">
                                                            <h5 class="title mb10">Food Delviery Mobile App</h5>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-place fz16 vam text-thm2 me-1"></i>
                                                                London, UK</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm pe-1"><i
                                                                    class="flaticon-30-days fz16 vam text-thm2 me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                2 hours ago</p>
                                                            <p class="mb-0 fz14 list-inline-item mb5-sm text-thm"><i
                                                                    class="flaticon-contract fz16 vam me-1 bdrl1 pl15 pl0-xs bdrn-xs"></i>
                                                                1 Received</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </th>
                                            <td class="vam"><span class="fz15 fw400">Web & App Design</span></td>
                                            <td class="vam"><span class="fz14 fw400">$500.00/Fixed</span></td>
                                            <td>
                                                <div class="d-flex">
                                                    <a href="" class="icon me-2" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Edit"><span
                                                            class="flaticon-pencil"></span></a>
                                                    <a href="" class="icon" data-bs-toggle="tooltip"
                                                        data-bs-placement="top" title="Delete"><span
                                                            class="flaticon-delete"></span></a>
                                                </div>
                                            </td>
                                        </tr>
                                    </tbody>
                                </table>
                                <div class="mbp_pagination text-center mt30">
                                    <ul class="page_navigation">
                                        <li class="page-item">
                                            <a class="page-link" href="#"> <span class="fas fa-angle-left"></span></a>
                                        </li>
                                        <li class="page-item"><a class="page-link" href="#">1</a></li>
                                        <li class="page-item active" aria-current="page">
                                            <a class="page-link" href="#">2 <span class="sr-only">(current)</span></a>
                                        </li>
                                        <li class="page-item"><a class="page-link" href="#">3</a></li>
                                        <li class="page-item"><a class="page-link" href="#">4</a></li>
                                        <li class="page-item"><a class="page-link" href="#">5</a></li>
                                        <li class="page-item"><a class="page-link" href="#">...</a></li>
                                        <li class="page-item"><a class="page-link" href="#">20</a></li>
                                        <li class="page-item">
                                            <a class="page-link" href="#"><span class="fas fa-angle-right"></span></a>
                                        </li>
                                    </ul>
                                    <p class="mt10 mb-0 pagination_page_count text-center">1 – 20 of 300+ property
                                        available</p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>