<template>
    <div class="tw-max-w-md tw-mx-auto tw-bg-white tw-rounded-xl tw-shadow-md tw-overflow-hidden tw-md:max-w-2xl tw-p-6">
      <h2 class="tw-text-2xl tw-font-bold tw-mb-4 tw-text-gray-800">Skill Keyword Rental</h2>
      
      <!-- Search Section -->
      <div class="tw-mb-6">
        <label for="keyword" class="tw-block tw-text-sm tw-font-medium tw-text-gray-700 tw-mb-2">Search Skill Keyword</label>
        <div class="tw-flex">
          <input 
            type="text" 
            id="keyword" 
            v-model="keyword" 
            @keyup.enter="searchKeyword"
            class="tw-flex-grow tw-rounded-l-md tw-border tw-border-gray-300 tw-shadow-sm tw-px-4 tw-py-2 tw-focus:outline-none tw-focus:ring-2 tw-focus:ring-indigo-500"
            placeholder="Enter a skill keyword"
          >
          <button 
            @click="searchKeyword"
            class="tw-bg-indigo-600 tw-text-white tw-px-4 tw-py-2 tw-rounded-r-md tw-hover:bg-indigo-700 tw-focus:outline-none tw-focus:ring-2 tw-focus:ring-indigo-500"
          >
            Search
          </button>
        </div>
      </div>
  
      <!-- Price Display Section -->
      <div v-if="price !== null" class="tw-mb-6">
        <h3 class="tw-text-lg tw-font-semibold tw-mb-2 tw-text-gray-700">Current Price:</h3>
        <p class="tw-text-3xl tw-font-bold tw-text-indigo-600">{{ formatPrice(price) }} / month</p>
      </div>
  
      <!-- Rental Section -->
      <div v-if="price !== null" class="tw-mb-6">
        <label for="months" class="tw-block tw-text-sm tw-font-medium tw-text-gray-700 tw-mb-2">Rental Duration (months)</label>
        <input 
          type="number" 
          id="months" 
          v-model="months" 
          min="1"
          class="tw-w-full tw-rounded-md tw-border tw-border-gray-300 tw-shadow-sm tw-px-4 tw-py-2 tw-focus:outline-none tw-focus:ring-2 tw-focus:ring-indigo-500"
        >
      </div>
  
      <!-- Total Price Display -->
      <div v-if="price !== null && months > 0" class="tw-mb-6">
        <h3 class="tw-text-lg tw-font-semibold tw-mb-2 tw-text-gray-700">Total Price:</h3>
        <p class="tw-text-2xl tw-font-bold tw-text-green-600">{{ formatPrice(totalPrice) }}</p>
      </div>
  
      <!-- Rent Button -->
      <button 
        v-if="price !== null && months > 0"
        @click="rentKeyword"
        class="tw-w-full tw-bg-green-600 tw-text-white tw-px-4 tw-py-2 tw-rounded-md tw-hover:bg-green-700 tw-focus:outline-none tw-focus:ring-2 tw-focus:ring-green-500"
      >
        Rent Keyword
      </button>
  
      <!-- Status Message -->
      <p v-if="statusMessage" class="tw-mt-4 tw-text-sm tw-text-gray-600">{{ statusMessage }}</p>
    </div>
  </template>
  
  <script setup>
  import { ref, computed } from 'vue';
import { toast } from 'vue3-toastify';
import { createActor } from '../../../declarations/aublisk_backend';
import { APP_CANISTER_ID, NATIVE_TOKEN_DECIMAL, NATIVE_TOKEN_SYMBOL } from '@/config';
  
  const keyword = ref('');
  const price = ref(null);
  const months = ref(1);
  const statusMessage = ref('');
  
  // Simulated API call to get keyword price
  const searchKeyword = async () => {
    if (keyword.value.trim() === '') {
      statusMessage.value = 'Please enter a keyword.';
      return;
    }


    if(!window.identity) {
      toast.error("You are not authenticated")
      return
    }

    let actor = createActor(APP_CANISTER_ID, {
      agentOptions : {
        //@ts-ignore
        identity: window.identity
      }
    })

    let action = actor.price_of_skillset(keyword.value)

    toast.promise(action, {
      pending: "Fetching Price of Keyword",
      error: "Error Fetching Price",
      success: "Success"
    }).then(price_data => {
      let human_readable = Number(price_data) / 10 ** NATIVE_TOKEN_DECIMAL
      price.value = human_readable
    })
    
    statusMessage.value = `Price found for "${keyword.value}".`;
  };
  
  const totalPrice = computed(() => {
    return price.value * months.value;
  });
  
  const formatPrice = (value) => {
    return `${value} ${NATIVE_TOKEN_SYMBOL}`
  };
  
  const rentKeyword = () => {
    if(!window.identity) {
      toast.error("You are not authenticated")
      return
    }

    let actor = createActor(APP_CANISTER_ID, {
      agentOptions : {
        //@ts-ignore
        identity: window.identity
      }
    })

    let action = actor.rent_skillset(keyword.value, months.value)

    toast.promise(action, {
      "error" : "Error Purchasing Skill",
      "pending": "PurChasing skill",
      "success": "Success"
    }).then(data_result => {
      if("Err" in data_result) {
        toast.error(data_result.Err)
      }
    })
    statusMessage.value = `You have rented "${keyword.value}" for ${months.value} months at a total cost of ${formatPrice(totalPrice.value)}.`;
    // Here you would typically make an API call to actually rent the keyword
  };
  </script>