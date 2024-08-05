<template>
    <div v-if="project_state" class="container">
        <div class="row wrap">
            <div class="col-lg-8">
                <div class="column">
                    <div
                        class="freelancer-single-v1 pt60 pb60 bdrs16 position-relative overflow-hidden d-flex align-items-center">
                        <!-- <img class="left-top-img wow zoomIn" src="@/static/images/vector-img/left-top.png" alt=""> -->
                        <!-- <img class="right-bottom-img wow zoomIn" src="@/static/images/vector-img/right-bottom.png" -->
                            <!-- alt=""> -->
                        <div class="row wow fadeInUp">
                            <div class="col-xl-12">
                                <div class="position-relative pl60 pl20-sm">
                                    <h2>{{ project_state?.title }}</h2>
                                    <div class="list-meta mt15">
                                        <p class="mb-0 dark-color fz15 fw500 list-inline-item mb5-sm"><i
                                                class="flaticon-place vam fz20 text-thm2 me-2"></i> {{ token_info?.symbol }}</p>
                                        <p class="mb-0 dark-color fz15 fw500 list-inline-item ml15 mb5-sm ml0-xs"><i
                                                class="flaticon-calendar text-thm2 vam fz20 me-2"></i> {{ formatDistance(new Date(Number(project_state?.timestamp || Date.now())), new Date()) }}
                                        </p>
                                        <!-- <p class="mb-0 dark-color fz15 fw500 list-inline-item ml15 mb5-sm ml0-xs"><i
                                                class="flaticon-website text-thm2 vam fz20 me-2"></i> 902 Views</p> -->
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                    <!-- Mini Data -->
                    <!-- <div class="row mt30">
                <div class="col-sm-6 col-xl-4">
                  <div class="iconbox-style1 contact-style d-flex align-items-start mb30">
                    <div class="icon flex-shrink-0"><span class="flaticon-notification-1"></span></div>
                    <div class="details">
                      <h5 class="title">Seller Type</h5>
                      <p class="mb-0 text">Company</p>
                    </div>
                  </div>
                </div>
                <div class="col-sm-6 col-xl-4">
                  <div class="iconbox-style1 contact-style d-flex align-items-start mb30">
                    <div class="icon flex-shrink-0"><span class="flaticon-dollar"></span></div>
                    <div class="details">
                      <h5 class="title">Project type</h5>
                      <p class="mb-0 text">Hourly</p>
                    </div>
                  </div>
                </div>
                <div class="col-sm-6 col-xl-4">
                  <div class="iconbox-style1 contact-style d-flex align-items-start mb30">
                    <div class="icon flex-shrink-0"><span class="flaticon-fifteen"></span></div>
                    <div class="details">
                      <h5 class="title">Project Duration</h5>
                      <p class="mb-0 text">10-15 Hours</p>
                    </div>
                  </div>
                </div>
                <div class="col-sm-6 col-xl-4">
                  <div class="iconbox-style1 contact-style d-flex align-items-start mb30">
                    <div class="icon flex-shrink-0"><span class="flaticon-like-1"></span></div>
                    <div class="details">
                      <h5 class="title">Project Level</h5>
                      <p class="mb-0 text">Expensive</p>
                    </div>
                  </div>
                </div>
                <div class="col-sm-6 col-xl-4">
                  <div class="iconbox-style1 contact-style d-flex align-items-start mb30">
                    <div class="icon flex-shrink-0"><span class="flaticon-translator"></span></div>
                    <div class="details">
                      <h5 class="title">Languages</h5>
                      <p class="mb-0 text">20</p>
                    </div>
                  </div>
                </div>
                <div class="col-sm-6 col-xl-4">
                  <div class="iconbox-style1 contact-style d-flex align-items-start mb30">
                    <div class="icon flex-shrink-0"><span class="flaticon-goal"></span></div>
                    <div class="details">
                      <h5 class="title">English Level</h5>
                      <p class="mb-0 text">Professional</p>
                    </div>
                  </div>
                </div>
              </div> -->
                    <div class="service-about">
                        <h4>Description</h4>
                        <div v-html="project_state?.description"></div>
                        <!-- <h4 class="mb30">Attachments</h4>
                <div class="row">
                  <div class="col-6 col-lg-3">
                    <div class="project-attach">
                      <h6 class="title">Project Brief</h6>
                      <p>PDF</p>
                      <span class="icon flaticon-page"></span>
                    </div>
                  </div>
                  <div class="col-6 col-lg-3">
                    <div class="project-attach">
                      <h6 class="title">Project Brief</h6>
                      <p>PDF</p>
                      <span class="icon flaticon-page"></span>
                    </div>
                  </div>
                </div> -->
                        <!-- <hr class="opacity-100 mb60 mt30"> -->
                        <h4 class="mb30">Skills Required</h4>
                        <div class="mb60">
                            <div v-for="skill in project_state?.skill_sets" class="tag list-inline-item mb-2 mb-xl-0 mr10">{{ skill }}</div>
                            
                        </div>


                        <div class="bsp_reveiw_wrt mt25">
                            <h4>Send Your Proposal</h4>
                            <CKEditor.component style="height: 208px;" :editor="editor" v-model="editorData"
                                :config="editorConfig" />
                            <div class="col-md-12 tw-mt-8">
                                <div class="d-grid">
                                    <button @click="submit_proposal" class="ud-btn btn-thm">Submit a Proposal<i
                                            class="fal fa-arrow-right-long"></i></button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <div class="col-lg-4">
                <div class="column">
                    <div class="blog-sidebar ms-lg-auto">
                        <div class="price-widget pt25 bdrs8">
                            <h3 class="widget-title">{{ function(){
                                let human_amount = Number(project_state?.amount_funded || 0) / 10 ** (token_info?.decimals || 1)
                                return human_amount
                            }() }} {{ token_info?.symbol }}</h3>
                            <p class="text fz14">Budget</p>
                            <div class="d-grid">
                                <button @click="submit_proposal" class="ud-btn btn-thm">Submit a Proposal<i
                                        class="fal fa-arrow-right-long"></i></button>
                            </div>
                        </div>

                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getQueryParam, getTokenInfo } from '../utils';
import { toast } from 'vue3-toastify';
import { createActor } from '../../../declarations/aublisk_backend';
import { APP_CANISTER_ID } from '../config';
import CKEditor from '@ckeditor/ckeditor5-vue';
import {
    ClassicEditor,
    Bold,
    Essentials,
    Heading,
    Indent,
    IndentBlock,
    Italic,
    Link,
    List,
    MediaEmbed,
    Paragraph,
    Table,
    Undo
} from 'ckeditor5';

import 'ckeditor5/ckeditor5.css';
import { Project } from '../../../declarations/aublisk_backend/aublisk_backend.did';
import { TokenInfo } from '../utils/types';
import { formatDistance } from 'date-fns';

let editor = ClassicEditor
let editorData = ref("")
let project_state = ref<Project|null>(null)
let token_info = ref<TokenInfo|null>(null)
let editorConfig = {
    toolbar: [
        'undo', 'redo', '|',
        'heading', '|', 'bold', 'italic', '|',
        'link', 'insertTable', 'mediaEmbed', '|',
        'bulletedList', 'numberedList', 'indent', 'outdent'
    ],
    plugins: [
        Bold,
        Essentials,
        Heading,
        Indent,
        IndentBlock,
        Italic,
        Link,
        List,
        MediaEmbed,
        Paragraph,
        Table,
        Undo
    ],
}
onMounted(() => {
    let id = getQueryParam("project_id")
    if (!id) {
        toast.error("No Project id was specified")
        return
    }

    let actor = createActor(APP_CANISTER_ID)
    let action = actor.read_project_and_status(id as string)
    toast.promise(action, {
        pending: "Fetching Project",
        "error": "Error Fetching Project",
        success: "Success"
    }).then(data => {
        let result = data[0]
        if(!result) {
            window.location.href = "/404"
            return
        }

        let [project, status] = result
        project_state.value = project
        getTokenInfo(project.icrc_token_address).then(data => {
            token_info.value = data
        })
    })

})

function submit_proposal() {
    if(!window.identity) {
        toast.error("You are not Authorized")
        return
    }
    let actor = createActor(APP_CANISTER_ID, {
        agentOptions: {
            identity: window.identity
        }
    })

    if(!project_state.value) {
        return toast.error("Project Not Found Yet")
    }

    let action = actor.create_project_proposal(project_state.value!!.hash_id, {
        sender: window.identity.getPrincipal(),
        content: editorData.value,
        timestamp: BigInt(0),
        attachments: []
    })

    toast.promise(action, {
      pending: "Sending Proposal",
      error: "Error Sending Proposal",
      success: "Success"
    }).then(data => {
      if("Err" in data) {
        toast.error(data.Err)
      }
    })
}
</script>
<style lang="css">
.ck-editor__editable_inline {
    min-height: 400px;
}
</style>