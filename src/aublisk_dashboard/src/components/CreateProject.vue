<script setup lang="ts">
import { onMounted, ref, watch } from 'vue';
import MobileNav from './MobileNav.vue';
import Select from "@/widgets/Select.vue";
import Switch from "@/widgets/Switch.vue";
import MSelect from "@/widgets/MultiSelect.vue"
import { Project } from '../../../declarations/aublisk_backend/aublisk_backend.did';
import { Principal } from '@dfinity/principal';
import Quill from 'quill';
import "quill/dist/quill.snow.css";
import { AuthClient } from "@dfinity/auth-client"
import { createActor } from '../../../declarations/aublisk_backend';
import { APP_CANISTER_ID } from '@/config';
import { SUPPORTED_TOKENS, TokenInfo } from '@/utils/types';
import { getTokenInfo } from '@/utils';
import { QuestionMarkCircleIcon } from "@heroicons/vue/24/outline"
import { toast } from 'vue3-toastify';
onMounted(() => {
 
  let editor = new Quill("#editor", {

    theme: 'snow',
  })

  window.editor = editor

  if(!window.identity) {
    return toast.error("You are not authenticated")
  }
  let actor = createActor(APP_CANISTER_ID, {
    agentOptions : {
      identity: window.identity
    }
  })

  actor.get_system_skillsets().then(skills => {
    skillsdata.value = skills
  })

})

function onSubmit(ev: Event) {

  if (!window.identity) {
    return toast.error("You are not authenticated")
  }

  let actor = createActor(APP_CANISTER_ID, {
    agentOptions: {
      identity: window.identity
    }
  })

  let project = project_data.value;

  //@ts-ignore
  let budget = Number(ev.target["budget"])

  if (isNaN(budget)) {
    return toast.error("Invalid Budget, its not a number")
  }

  if (!custom_token_info.value) {
    return toast.error("Payment Token is not yet determined")
  }

  project.amount_funded = BigInt(budget * (10 ** (custom_token_info.value.decimals)))
  //@ts-ignore
  project.description = window.editor.root.innerHTML;
  project.icrc_token_address = custom_token_info.value.canister_id

  project.skill_sets = selected_skills.value


  //@ts-ignore
  toast.promise(actor.create_project(project, project_contact_link.value), {
    pending: "Calling Blockchain",
    error: "Error calling blockchain",
    success: "Success"
  }).then(response => {
    if ("Ok" in response) {
      let hash_id = response.Ok
      // move to funding page.
    } else if("Err" in response) {
      toast.error(response.Err)
    }
  })

}


let project_data = ref<Project>({
  title: '',
  icrc_token_address: '',
  hash_id: '',
  //@ts-ignore
  owner: Principal.anonymous(),
  description: '',
  amount_funded: 0n,
  skill_sets: [],
  milestones: []
})

let show_custom_token = ref(false)
watch(show_custom_token, (nV, oV) => {
  custom_token_info.value = null
})
let selected_skills = ref([])
let project_contact_link = ref("")
let custom_token_id = ref("")
let custom_token_info = ref<TokenInfo | null>(null)
let fetching_token_info = ref(false)
let skillsdata = ref<string[]>(["rust", "java", "character"])
function onCustomTokenInputChange() {
  if (custom_token_id.value.length == 27) {
    fetching_token_info.value = true
    getTokenInfo(custom_token_id.value).then(data => {
      custom_token_info.value = data
    }).finally(() => {
      fetching_token_info.value = false
    })
  }
}
</script>

<template>
  <div class="row pb40">
    <div class="col-lg-12">
      <div class="dashboard_navigationbar d-block d-lg-none">
        <MobileNav />
      </div>
    </div>
    <div class="col-lg-12 tw-flex tw-justify-between">
      <div class="dashboard_title_area">
        <h2 class="tw-font-bold tw-text-3xl">Create Project</h2>
        <p class="text">Get paid for creating project</p>
      </div>
     

    </div>
  </div>
  <div class="row">
    <div class="col-xl-12">
      <div class="ps-widget bgc-white bdrs4 p30 mb30 overflow-hidden position-relative">
        <div class="bdrb1 pb15 mb25">
          <h5 class="list-title">Basic Information</h5>
        </div>
        <div class="col-xl-8">
          <form @submit.prevent="onSubmit" class="form-style1">
            <div class="row">
              <div class="col-sm-12">
                <div class="mb20">
                  <label class="heading-color ff-heading fw500 mb10">Project Title</label>
                  <input required v-model.trim="project_data.title" type="text" class="form-control"
                    placeholder="Project Title">
                </div>
              </div>

              <div class="col-sm-12">
                <div class="mb20">
                  <label class="heading-color ff-heading fw500 mb10">Project Contact Link</label>
                  <input required v-model.trim="project_contact_link" type="text" class="form-control"
                    placeholder="Link used to contact you and chat">
                </div>
              </div>

              <div class="col-sm-6">
                <div class="mb20">
                  <div>
                    <div class="tw-flex tw-justify-between tw-items-center">
                      <div><label class="heading-color ff-heading fw500">Payment Token</label></div>
                      <div class="tw-flex tw-flex-col tw-items-end"><span class="tw-text-xs">Use unsupported
                          token</span>
                        <Switch v-model="show_custom_token" />
                      </div>
                    </div>
                    <div v-if="show_custom_token">

                      <div class="tw-relative tw-mt-2 tw-rounded-md tw-shadow-sm">
                        <div
                          class="tw-pointer-events-none tw-absolute tw-inset-y-0 tw-left-0 tw-flex tw-items-center tw-pl-3">
                          <svg v-if="fetching_token_info"
                            class="tw-animate-spin tw--ml-1 tw-mr-3 tw-h-5 tw-w-5 tw-text-accent"
                            xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                            <circle class="tw-opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4">
                            </circle>
                            <path class="tw-opacity-75" fill="currentColor"
                              d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z">
                            </path>
                          </svg>
                          <img v-else-if="custom_token_info?.logo_url" :src="custom_token_info?.logo_url"
                            class="tw-h-5 tw-w-5 tw-text-gray-400" aria-hidden="true" />

                          <QuestionMarkCircleIcon v-else class="tw-w-5 tw-h-5" />
                        </div>
                        <input required @input="onCustomTokenInputChange" type="text"
                          name="custom_icrc_token"
                          class="tw-block tw-w-full tw-rounded-md tw-border-0 tw-py-3 tw-pl-10 tw-text-gray-900 tw-ring-1 tw-ring-inset tw-ring-gray-300 tw-placeholder:text-gray-400 tw-focus:ring-2 tw-focus:ring-inset tw-focus:ring-indigo-600 tw-sm:text-sm tw-sm:leading-6"
                          placeholder="7p3gx-jaaaa-aaaal-acbda-cai" />
                      </div>
                    </div>
                    <Select logo-key="logo_url" value-key="canister_id" name-key="name" v-model="custom_token_info" :options="SUPPORTED_TOKENS" v-else />
                  </div>
                </div>
              </div>

              <div class="col-sm-6">
                <div class="mb20">
                  <label class="heading-color ff-heading fw500 mb10">Budget</label>
                  <input required :disabled="!custom_token_info" v-model.number="project_data.amount_funded"
                    type="number" class="form-control" name="budget" placeholder="Amount in token">
                </div>
              </div>


              <div class="col-sm-12">
                <div class="mb20">
                  <div class="form-style1">
                    <label class="heading-color ff-heading fw500 mb10">Skills</label>
                    <MSelect v-model="selected_skills" :options="skillsdata" />
                    <!-- <div class="bootselect-multiselect">
                              <select class="selectpicker" multiple>
                                <option>Figma</option>
                                <option>Adobe XD</option>
                                <option>CSS</option>
                                <option>HTML</option>
                                <option>Bootstrap</option>
                              </select>
                            </div> -->
                  </div>
                </div>
              </div>
              <div class="col-md-12">
                <div class="mb10">
                  <label class="heading-color ff-heading fw500 mb10">Project Detail</label>
                  <div>
                    <div class="tw-w-full">

                      <div id="editor" class="tw-prose tw-max-w-full tw-h-96"></div>
                    </div>
                  </div>
                </div>
              </div>
              <div class="col-md-12">
                <div class="text-start">
                  <button class="ud-btn btn-thm" >Create Project<i class="fal fa-arrow-right-long"></i></button>
                </div>
              </div>
            </div>
          </form>
        </div>
      </div>
      <!-- <div class="ps-widget bgc-white bdrs12 p30 mb30 overflow-hidden position-relative">
        <div class="bdrb1 pb15 mb25">
          <h5 class="list-title">Upload Attachments</h5>
        </div>
        <div class="row">
          <div class="col-6 col-xl-2">
            <div class="project-attach">
              <h6 class="title">Project Brief</h6>
              <p>PDF</p>
              <span class="icon flaticon-page"></span>
            </div>
          </div>
          <div class="col-6 col-xl-2">
            <div class="project-attach">
              <h6 class="title">Project Brief</h6>
              <p>PDF</p>
              <span class="icon flaticon-page"></span>
            </div>
          </div>
          <div class="col-6 col-xl-3">
            <a class="upload-img" href="">Upload Files</a>
          </div>
        </div>
        <p class="text">Maximum file size: 10 MB</p>
        <div class="text-start">
          <a class="ud-btn btn-thm" href="page-contact.html">Save & Publish<i class="fal fa-arrow-right-long"></i></a>
        </div>
      </div> -->
    </div>
  </div>
</template>