<template>
  <div class="tw-max-w-md tw-mx-auto tw-bg-white tw-rounded-xl tw-shadow-md tw-overflow-hidden tw-md:max-w-2xl">
    <div class="tw-md:flex">
      <div class="tw-p-8 tw-w-full">
        <div class="tw-uppercase tw-tracking-wide tw-text-sm tw-text-indigo-500 tw-font-semibold tw-mb-4">Staking
          Interface</div>
        <div class="tw-flex tw-mb-4">
          <button @click="activeTab = 'stake'" :class="['tw-flex-1 tw-py-2 tw-px-4 tw-text-center tw-focus:outline-none',
            activeTab === 'stake' ? 'tw-bg-indigo-500 tw-text-white' : 'tw-bg-gray-200 tw-text-gray-700']">
            Stake
          </button>
          <button @click="activeTab = 'unstake'"
            :class="['tw-flex-1 tw-py-2 tw-px-4 tw-text-center tw-focus:outline-none',
              activeTab === 'unstake' ? 'tw-bg-indigo-500 tw-text-white' : 'tw-bg-gray-200 tw-text-gray-700']">
            Unstake
          </button>
        </div>

        <div v-if="activeTab === 'stake'">
          <div class="tw-mb-4">
            <label class="tw-block tw-text-gray-700 tw-text-sm tw-font-bold tw-mb-2" for="amount">
              Amount to Stake
            </label>
            <div class="tw-flex">
              <input
                class="tw-shadow tw-appearance-none tw-border tw-rounded-l tw-w-full tw-py-2 tw-px-3 tw-text-gray-700 tw-leading-tight tw-focus:outline-none tw-focus:shadow-outline"
                id="amount" type="number" v-model="stakedAmount" placeholder="Enter amount">
              <button @click="setMaxStake"
                class="tw-bg-gray-300 tw-hover:bg-gray-400 tw-text-gray-800 tw-font-bold tw-py-2 tw-px-4 tw-rounded-r tw-focus:outline-none tw-focus:shadow-outline">
                Max
              </button>
            </div>
            <p class="tw-text-gray-600 tw-text-xs tw-italic tw-mt-1">
              Available balance: {{ formatBalance(walletBalance) }} {{ NATIVE_TOKEN_SYMBOL }}
            </p>
          </div>

          <div class="tw-mb-6">
            <label class="tw-block tw-text-gray-700 tw-text-sm tw-font-bold tw-mb-2" for="duration">
              Staking Duration
            </label>
            <select
              class="tw-block tw-appearance-none tw-w-full tw-bg-white tw-border tw-border-gray-400 tw-hover:border-gray-500 tw-px-4 tw-py-2 tw-pr-8 tw-rounded tw-shadow tw-leading-tight tw-focus:outline-none tw-focus:shadow-outline"
              id="duration" v-model="stakeDuration">
              <option value="30">30 Days</option>
              <option value="60">60 Days</option>
              <option value="90">90 Days</option>
            </select>
          </div>

          <div class="tw-flex tw-items-center tw-justify-between">
            <button
              class="tw-bg-indigo-500 tw-hover:bg-indigo-700 tw-text-white tw-font-bold tw-py-2 tw-px-4 tw-rounded tw-focus:outline-none tw-focus:shadow-outline"
              type="button" @click="handleStake">
              Stake Now
            </button>
          </div>
        </div>

        <div v-else>
          <div class="tw-mb-6">
            <h3 class="tw-text-lg tw-font-semibold tw-text-gray-700 tw-mb-2">Currently Staked</h3>
            <p class="tw-text-2xl tw-font-bold tw-text-indigo-600">{{ formatBalance(stakedAmount) }}</p>
          </div>

          <div class="tw-mb-4">
            <label class="tw-block tw-text-gray-700 tw-text-sm tw-font-bold tw-mb-2" for="unstakeAmount">
              Amount to Unstake
            </label>
            <div class="tw-flex">
              <input
                class="tw-shadow tw-appearance-none tw-border tw-rounded-l tw-w-full tw-py-2 tw-px-3 tw-text-gray-700 tw-leading-tight tw-focus:outline-none tw-focus:shadow-outline"
                id="unstakeAmount" type="number" v-model="unstakeAmount" placeholder="Enter amount to unstake"
                :max="stakedAmount">
              <button @click="setMaxUnstake"
                class="tw-bg-gray-300 tw-hover:bg-gray-400 tw-text-gray-800 tw-font-bold tw-py-2 tw-px-4 tw-rounded-r tw-focus:outline-none tw-focus:shadow-outline">
                Max
              </button>
            </div>
          </div>

          <div class="tw-flex tw-items-center tw-justify-between">
            <button
              class="tw-bg-red-500 tw-hover:bg-red-700 tw-text-white tw-font-bold tw-py-2 tw-px-4 tw-rounded tw-focus:outline-none tw-focus:shadow-outline"
              type="button" @click="handleUnstake" :disabled="!canUnstake">
              Unstake
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { APP_CANISTER_ID, NATIVE_TOKEN_DECIMAL, NATIVE_TOKEN_ID, NATIVE_TOKEN_SYMBOL } from '@/config';
import { get_wallet_balance } from '@/utils';
import { ref, computed, onMounted } from 'vue';
import { toast } from 'vue3-toastify';
import { createActor } from '../../../declarations/aublisk_backend';



// Reactive state
const activeTab = ref('stake');
const stakeDuration = ref('30');
const walletBalance = ref(0); // Example balance, replace with actual wallet integration
const stakedAmount = ref(0); // Example staked amount, replace with actual staked amount
const unstakeAmount = ref('');

onMounted(() => {
  if (!window.identity) return
  get_wallet_balance(window.identity.getPrincipal(), NATIVE_TOKEN_ID).then(balance => {
    let user_readable_balance = balance / 10 ** (NATIVE_TOKEN_DECIMAL)
    walletBalance.value = user_readable_balance
  })
})
// Computed properties
const canUnstake = computed(() => {
  return unstakeAmount.value > 0 && unstakeAmount.value <= stakedAmount.value;
});

// Methods
const formatBalance = (balance) => {
  return balance
};

const setMaxStake = () => {
  stakedAmount.value = walletBalance.value
};

const setMaxUnstake = () => {
  unstakeAmount.value = stakedAmount.value.toString();
};

const handleStake = () => {
  if (!window.identity) {
    toast.error("You are not Authenticated")
    return
  }

  let actor = createActor(APP_CANISTER_ID, {
    agentOptions: {
      //@ts-ignore
      identity: window.identity
    }
  })

  //TODO for now we use 30 days as dissolving delay
  let action = actor.stake(BigInt(stakedAmount.value), BigInt(parseInt(stakeDuration.value)))
  toast.promise(action, {
    "error": "Error Trying to Stake",
    "pending": "Calling Staking Routine",
    'success': "Success"
  }).then(data => {
    if ("Err" in data) {
      toast.error(data.Err)
    }
  })
  // Implement staking logic here
  console.log(`Staking ${stakeAmount.value} for ${stakeDuration.value} days`);
};

const handleUnstake = () => {
  if (!window.identity) {
    toast.error("You are not Authenticated")
    return
  }

  let actor = createActor(APP_CANISTER_ID, {
    agentOptions: {
      //@ts-ignore
      identity: window.identity
    }
  })

  //TODO for now we use 30 days as dissolving delay
  let action = actor.unstake()
  toast.promise(action, {
    "error": "Error Trying to Unstake",
    "pending": "Calling unstaking Routine",
    'success': "Success"
  }).then(data => {

  })
  // Implement unstaking logic here
  console.log(`Unstaking ${unstakeAmount.value}`);
};
</script>