<template>
    <TransitionRoot appear :show="isOpen" as="template">
      <Dialog as="div" @close="closeModal" class="tw-relative tw-z-10">
        <TransitionChild
          as="template"
          enter="tw-duration-300 tw-ease-out"
          enter-from="tw-opacity-0"
          enter-to="tw-opacity-100"
          leave="tw-duration-200 tw-ease-in"
          leave-from="tw-opacity-100"
          leave-to="tw-opacity-0"
        >
          <div class="tw-fixed tw-inset-0 tw-bg-black tw-bg-opacity-25" />
        </TransitionChild>
  
        <div class="tw-fixed tw-inset-0 tw-overflow-y-auto">
          <div class="tw-flex tw-min-h-full tw-items-center tw-justify-center tw-p-4 tw-text-center">
            <TransitionChild
              as="template"
              enter="tw-duration-300 tw-ease-out"
              enter-from="tw-opacity-0 tw-scale-95"
              enter-to="tw-opacity-100 tw-scale-100"
              leave="tw-duration-200 tw-ease-in"
              leave-from="tw-opacity-100 tw-scale-100"
              leave-to="tw-opacity-0 tw-scale-95"
            >
              <DialogPanel class="tw-w-full tw-max-w-md tw-transform tw-overflow-hidden tw-rounded-2xl tw-bg-white tw-p-6 tw-text-left tw-align-middle tw-shadow-xl tw-transition-all">
                <DialogTitle as="h3" class="tw-text-lg tw-font-medium tw-leading-6 tw-text-gray-900">
                  Send Funds
                </DialogTitle>
                <div class="tw-mt-2">
                  <div class="tw-text-sm tw-text-gray-500 tw-mb-2">
                    Maximum amount: {{ formatAmount(maxAmount) }} {{ currencySymbol }}
                  </div>
                  <div class="tw-relative">
                    <input
                      v-model="amount"
                      type="number"
                      placeholder="Amount"
                      class="tw-w-full tw-px-3 tw-py-2 tw-border tw-border-gray-300 tw-rounded-md tw-focus:outline-none tw-focus:ring-2 tw-focus:ring-blue-500"
                    />
                    <button
                      @click="setMaxAmount"
                      class="tw-absolute tw-right-2 tw-top-1/2 tw-transform tw--translate-y-1/2 tw-px-2 tw-py-1 tw-text-xs tw-bg-gray-200 tw-rounded tw-hover:bg-gray-300"
                    >
                      Max
                    </button>
                  </div>
                  <input
                    v-model="walletAddress"
                    type="text"
                    placeholder="Wallet Address"
                    class="tw-w-full tw-mt-2 tw-px-3 tw-py-2 tw-border tw-border-gray-300 tw-rounded-md tw-focus:outline-none tw-focus:ring-2 tw-focus:ring-blue-500"
                  />
                  <div class="tw-mt-2 tw-text-sm tw-text-gray-500">
                    <p>Transfer fee: {{ formatAmount(transferFee) }} {{ currencySymbol }}</p>
                    <p>Network fee: {{ formatAmount(networkFee) }} {{ currencySymbol }}</p>
                    <p class="tw-font-medium tw-mt-1">
                      Actual amount to be sent: {{ formatAmount(actualAmount) }} {{ currencySymbol }}
                    </p>
                  </div>
                </div>
  
                <div class="tw-mt-4">
                  <button
                    type="button"
                    class="tw-w-full tw-inline-flex tw-justify-center tw-items-center tw-rounded-md tw-border tw-border-transparent tw-bg-black tw-px-4 tw-py-2 tw-text-sm tw-font-medium tw-text-white hover:tw-bg-gray-800 focus:tw-outline-none focus-visible:tw-ring-2 focus-visible:tw-ring-black focus-visible:tw-ring-offset-2"
                    @click="sendFunds"
                    :disabled="isSending || !isValidAmount"
                  >
                    <template v-if="isSending">
                      <svg class="tw-animate-spin tw--ml-1 tw-mr-3 tw-h-5 tw-w-5 tw-text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                        <circle class="tw-opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                        <path class="tw-opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                      </svg>
                      Sending...
                    </template>
                    <template v-else>
                      Send
                    </template>
                  </button>
                </div>
              </DialogPanel>
            </TransitionChild>
          </div>
        </div>
      </Dialog>
    </TransitionRoot>
  </template>
  
  <script setup lang="ts">
  import { ref, computed } from 'vue'
  import { Dialog, DialogPanel, DialogTitle, TransitionChild, TransitionRoot } from '@headlessui/vue'
  
  const props = defineProps<{
    isOpen: boolean
    maxAmount: number
    transferFee: number
    networkFee: number
    currencySymbol: string
  }>()
  
  const emit = defineEmits<{
    (e: 'close'): void
    (e: 'send', amount: number, address: string): void
  }>()
  
  const amount = ref('')
  const walletAddress = ref('')
  const isSending = ref(false)
  
  const closeModal = () => {
    emit('close')
  }
  
  const setMaxAmount = () => {
    amount.value = props.maxAmount.toString()
  }
  
  const actualAmount = computed(() => {
    const inputAmount = parseFloat(amount.value) || 0
    return Math.max(0, inputAmount - props.transferFee - props.networkFee)
  })
  
  const isValidAmount = computed(() => {
    const inputAmount = parseFloat(amount.value) || 0
    return inputAmount > 0 && inputAmount <= props.maxAmount && actualAmount.value > 0
  })
  
  const formatAmount = (value: number) => {
    return value.toFixed(8)  // Adjust decimal places as needed
  }
  
  const sendFunds = async () => {
    if (!isValidAmount.value || !walletAddress.value) return
  
    isSending.value = true
    try {
      // Simulating an async operation
      await new Promise(resolve => setTimeout(resolve, 2000))
      emit('send', actualAmount.value, walletAddress.value)
      closeModal()
    } catch (error) {
      console.error('Error sending funds:', error)
    } finally {
      isSending.value = false
    }
  }
  </script>