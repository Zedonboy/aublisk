<template>
  <TransitionRoot as="template" :show="isOpen">
    <Dialog as="div" class="tw-relative tw-z-10" @close="closeDialog">
      <TransitionChild as="template" enter="tw-ease-out tw-duration-300" enter-from="tw-opacity-0"
        enter-to="tw-opacity-100" leave="tw-ease-in tw-duration-200" leave-from="tw-opacity-100"
        leave-to="tw-opacity-0">
        <div class="tw-fixed tw-inset-0 tw-bg-gray-500 tw-bg-opacity-75 tw-transition-opacity" />
      </TransitionChild>
      <div class="tw-fixed tw-inset-0 tw-z-10 tw-overflow-y-auto">
        <div
          class="tw-flex tw-min-h-full tw-items-end tw-justify-center tw-p-4 tw-text-center sm:tw-items-center sm:tw-p-0">
          <TransitionChild as="template" enter="tw-ease-out tw-duration-300"
            enter-from="tw-opacity-0 tw-translate-y-4 sm:tw-translate-y-0 sm:tw-scale-95"
            enter-to="tw-opacity-100 tw-translate-y-0 sm:tw-scale-100" leave="tw-ease-in tw-duration-200"
            leave-from="tw-opacity-100 tw-translate-y-0 sm:tw-scale-100"
            leave-to="tw-opacity-0 tw-translate-y-4 sm:tw-translate-y-0 sm:tw-scale-95">
            <DialogPanel
              class="tw-relative tw-transform tw-overflow-hidden tw-rounded-lg tw-bg-white tw-px-4 tw-pb-4 tw-pt-5 tw-text-left tw-shadow-xl tw-transition-all sm:tw-my-8 sm:tw-w-full sm:tw-max-w-lg sm:tw-p-6">
              <div>
                <div class="tw-mt-3 sm:tw-mt-5">
                  <DialogTitle as="h3" class="tw-text-base tw-text-center tw-font-semibold tw-leading-6 tw-text-gray-900">
                    Payout
                  </DialogTitle>
                  <div class="tw-mt-2">
                    <Listbox as="div" v-model="selectedEmail">
                      <ListboxLabel class="tw-block tw-text-sm tw-font-medium tw-leading-6 tw-text-gray-900">Send to
                      </ListboxLabel>
                      <div class="tw-relative tw-mt-2">
                        <ListboxButton
                          class="tw-relative tw-w-full tw-cursor-default tw-rounded-md tw-bg-white tw-py-1.5 tw-pl-3 tw-pr-10 tw-text-left tw-text-gray-900 tw-shadow-sm tw-ring-1 tw-ring-inset tw-ring-gray-300 tw-focus:outline-none tw-focus:ring-2 tw-focus:ring-indigo-600 tw-sm:text-sm tw-sm:leading-6">
                          <span class="tw-block tw-truncate">{{ selectedEmail }}</span>
                          <span
                            class="tw-pointer-events-none tw-absolute tw-inset-y-0 tw-right-0 tw-flex tw-items-center tw-pr-2">
                            <ChevronUpDownIcon class="tw-h-5 tw-w-5 tw-text-gray-400" aria-hidden="true" />
                          </span>
                        </ListboxButton>

                        <transition leave-active-class="tw-transition tw-ease-in tw-duration-100"
                          leave-from-class="tw-opacity-100" leave-to-class="tw-opacity-0">
                          <ListboxOptions
                            class="tw-absolute tw-z-10 tw-mt-1 tw-max-h-60 tw-w-full tw-overflow-auto tw-rounded-md tw-bg-white tw-py-1 tw-text-base tw-shadow-lg tw-ring-1 tw-ring-black tw-ring-opacity-5 tw-focus:outline-none tw-sm:text-sm">
                            <ListboxOption as="template" v-for="(email, idx) in emails" :key="idx" :value="email"
                              v-slot="{ active, selected }">
                              <li
                                :class="[active ? 'tw-bg-indigo-600 tw-text-white' : 'tw-text-gray-900', 'tw-relative tw-cursor-default tw-select-none tw-py-2 tw-pl-3 tw-pr-9']">
                                <span
                                  :class="[selected ? 'tw-font-semibold' : 'tw-font-normal', 'tw-block tw-truncate']">{{
                                  email }}</span>

                                <span v-if="selected"
                                  :class="[active ? 'tw-text-white' : 'tw-text-indigo-600', 'tw-absolute tw-inset-y-0 tw-right-0 tw-flex tw-items-center tw-pr-4']">
                                  <CheckIcon class="tw-h-5 tw-w-5" aria-hidden="true" />
                                </span>
                              </li>
                            </ListboxOption>
                          </ListboxOptions>
                        </transition>
                      </div>
                    </Listbox>


                   
                    <div class="tw-mt-4">
                      <label for="amount" class="tw-block tw-text-sm tw-font-medium tw-text-gray-700">Amount</label>
                      <div class="tw-relative tw-mt-1 tw-rounded-md tw-shadow-sm">
                        <div
                          class="tw-pointer-events-none tw-absolute tw-inset-y-0 tw-left-0 tw-flex tw-items-center tw-pl-3">
                          <!-- <span class="tw-text-gray-500 sm:tw-text-sm">$</span> -->
                        </div>
                        <input type="number" name="amount" id="amount" v-model="amount"
                          class="tw-block tw-w-full tw-rounded-md tw-border-0 tw-py-1.5 tw-pl-3 tw-pr-12 tw-text-gray-900 tw-ring-1 tw-ring-inset tw-ring-gray-300 tw-placeholder:text-gray-400 tw-focus:ring-2 tw-focus:ring-inset tw-focus:ring-indigo-600 sm:tw-text-sm sm:tw-leading-6"
                          placeholder="0.00" aria-describedby="price-currency">
                        <div
                          class="tw-pointer-events-none tw-absolute tw-inset-y-0 tw-right-0 tw-flex tw-items-center tw-pr-3">
                          <span class="tw-text-gray-500 sm:tw-text-sm" id="price-currency">{{ token_symbol || "USD" }}</span>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
              <div class="tw-mt-5 sm:tw-mt-6 sm:tw-grid sm:tw-grid-flow-row-dense sm:tw-grid-cols-2 sm:tw-gap-3">
                <button type="button"
                  class="tw-inline-flex tw-w-full tw-justify-center tw-rounded-md tw-bg-indigo-600 tw-px-3 tw-py-2 tw-text-sm tw-font-semibold tw-text-white tw-shadow-sm hover:tw-bg-indigo-500 tw-focus-visible:outline tw-focus-visible:outline-2 tw-focus-visible:outline-offset-2 tw-focus-visible:outline-indigo-600 sm:tw-col-start-2"
                  @click="confirmPayout">
                  Pay
                </button>
                <button type="button"
                  class="tw-mt-3 tw-inline-flex tw-w-full tw-justify-center tw-rounded-md tw-bg-white tw-px-3 tw-py-2 tw-text-sm tw-font-semibold tw-text-gray-900 tw-shadow-sm tw-ring-1 tw-ring-inset tw-ring-gray-300 hover:tw-bg-gray-50 sm:tw-col-start-1 sm:tw-mt-0"
                  @click="closeDialog">
                  Cancel
                </button>
              </div>
            </DialogPanel>
          </TransitionChild>
        </div>
      </div>
    </Dialog>
  </TransitionRoot>
</template>

<script setup>
import { ref, defineProps, defineEmits } from 'vue'
import { Dialog, DialogPanel, DialogTitle, TransitionChild, TransitionRoot, Listbox, ListboxButton, ListboxOptions, ListboxOption } from '@headlessui/vue'
import { CheckIcon, ChevronUpDownIcon } from '@heroicons/vue/20/solid'

const props = defineProps({
  emails: {
    type: Array,
    required: true
  },

  token_symbol: {
    type: String,
    required : true
  }
})

const emit = defineEmits(['confirm', 'cancel', 'close'])

const isOpen = ref(true)
const selectedEmail = ref(props.emails[0])
const amount = ref('')

const closeDialog = () => {
  isOpen.value = false
  emit('close')
}

const confirmPayout = () => {
  if (selectedEmail.value && amount.value) {
    emit('confirm', { email: selectedEmail.value, amount: parseFloat(amount.value) })
    closeDialog()
  }
}

defineExpose({
  isOpen,
  closeDialog
})
</script>