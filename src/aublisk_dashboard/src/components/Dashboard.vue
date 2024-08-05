<script setup lang="ts">
import { onMounted, ref } from 'vue';
import MobileNav from './MobileNav.vue';
import { createActor } from '../../../declarations/aublisk_backend';
import { APP_CANISTER_ID } from '@/config';
import { toast } from 'vue3-toastify';
import { DashboardMetric } from '../../../declarations/aublisk_backend/aublisk_backend.did';
import Stake from '@/widgets/Stake.vue';
import SkillRent from '@/widgets/SkillRent.vue';
let metric_state = ref<DashboardMetric | null>(null)
onMounted(() => {
  if (!window.identity) {
    return toast.error("You are not authorized")
  }
  let actor = createActor(APP_CANISTER_ID, {
    agentOptions: {
      identity: window.identity
    }
  })

  try {
    toast.promise(actor.get_dashboard_metric(), {
      pending: "Fetching Dashboard Metric",
      error: "Error Fetching Dashboard Metric",
      success: "Success"
    }).then(metric => {
      metric_state.value = metric
    })
  } catch (error) {
    toast.error("There was an error")
  }
})
</script>

<template>
  <div class="row pb40">
    <div class="col-lg-12">
      <div class="dashboard_navigationbar d-block d-lg-none">
        <MobileNav />
      </div>
    </div>
    <div class="col-lg-12">
      <div class="dashboard_title_area">
        <h2 class="tw-font-bold tw-text-3xl">Dashboard</h2>
        <p class="text">Gm Anonymous</p>
      </div>
    </div>
  </div>
  <div class="row">
    <div class="col-sm-6 col-xxl-3">
      <div class="d-flex align-items-center justify-content-between statistics_funfact">
        <div class="details">
          <div class="fz15">Number of Projects</div>
          <div class="title">{{ Number(metric_state?.number_of_project || 0) }}</div>
          <!-- <div class="text fz14"><span class="text-thm">10</span> New Offered</div> -->
        </div>
        <div class="icon text-center"><i class="flaticon-contract"></i></div>
      </div>
    </div>
    <div class="col-sm-6 col-xxl-3">
      <div class="d-flex align-items-center justify-content-between statistics_funfact">
        <div class="details">
          <div class="fz15">Total Amount Received</div>
          <div class="title">{{ Number(metric_state?.total_amount_received || 0) }}</div>
          <!-- <div class="text fz14"><span class="text-thm">80+</span> New Completed</div> -->
        </div>
        <div class="icon text-center"><i class="flaticon-success"></i></div>
      </div>
    </div>
    <div class="col-sm-6 col-xxl-3">
      <div class="d-flex align-items-center justify-content-between statistics_funfact">
        <div class="details">
          <div class="fz15">Total Amount Spent</div>
          <div class="title">{{ Number(metric_state?.total_amount_spent || 0) }}</div>
          <!-- <div class="text fz14"><span class="text-thm">35+</span> New Queue</div> -->
        </div>
        <div class="icon text-center"><i class="flaticon-review"></i></div>
      </div>
    </div>
    <!-- <div class="col-sm-6 col-xxl-3">
      <div class="d-flex align-items-center justify-content-between statistics_funfact">
        <div class="details">
          <div class="fz15">Total Review</div>
          <div class="title">22,786</div>
          <div class="text fz14"><span class="text-thm">290+</span> New Review</div>
        </div>
        <div class="icon text-center"><i class="flaticon-review-1"></i></div>
      </div>
    </div> -->
  </div>
  <div class="row">
    <Stake/>
    <SkillRent/>
  </div>
 
</template>