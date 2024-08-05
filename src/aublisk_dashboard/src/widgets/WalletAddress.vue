<template>
    <div>
      <div class="tw-bg-white tw-rounded-lg tw-py-6 tw-max-w-md tw-w-full">
        <h2 class="tw-text-2xl tw-font-bold tw-mb-4 tw-text-center tw-text-gray-800">Project Fund Deposit Address</h2>
        <div class="tw-flex tw-items-center tw-bg-gray-100 tw-p-3 tw-rounded-md">
        
          <p class="tw-text-sm tw-break-all tw-font-mono tw-text-gray-800">{{ walletAddress }}</p>
          <Menu as="div" class="tw-relative tw-ml-auto">
            <MenuButton class="tw-text-gray-500 hover:tw-text-gray-700 focus:tw-outline-none">
              <DotsVerticalIcon class="tw-h-5 tw-w-5" aria-hidden="true" />
            </MenuButton>
            <transition
              enter-active-class="tw-transition tw-duration-100 tw-ease-out"
              enter-from-class="tw-transform tw-scale-95 tw-opacity-0"
              enter-to-class="tw-transform tw-scale-100 tw-opacity-100"
              leave-active-class="tw-transition tw-duration-75 tw-ease-in"
              leave-from-class="tw-transform tw-scale-100 tw-opacity-100"
              leave-to-class="tw-transform tw-scale-95 tw-opacity-0"
            >
              <MenuItems class="tw-absolute tw-right-0 tw-mt-2 tw-w-56 tw-origin-top-right tw-bg-white tw-rounded-md tw-shadow-lg tw-ring-1 tw-ring-black tw-ring-opacity-5 focus:tw-outline-none">
                <div class="tw-py-1">
                  <MenuItem v-slot="{ active }">
                    <button
                      @click="copyToClipboard"
                      :class="[
                        active ? 'tw-bg-gray-100 tw-text-gray-900' : 'tw-text-gray-700',
                        'tw-group tw-flex tw-items-center tw-px-4 tw-py-2 tw-text-sm tw-w-full',
                      ]"
                    >
                      <ClipboardCopyIcon
                        class="tw-mr-3 tw-h-5 tw-w-5 tw-text-gray-400 group-hover:tw-text-gray-500"
                        aria-hidden="true"
                      />
                      Copy Address
                    </button>
                  </MenuItem>
                </div>
              </MenuItems>
            </transition>
          </Menu>
        </div>
        <Transition
          enter-active-class="tw-transition tw-duration-100 tw-ease-out"
          enter-from-class="tw-transform tw-scale-95 tw-opacity-0"
          enter-to-class="tw-transform tw-scale-100 tw-opacity-100"
          leave-active-class="tw-transition tw-duration-75 tw-ease-in"
          leave-from-class="tw-transform tw-scale-100 tw-opacity-100"
          leave-to-class="tw-transform tw-scale-95 tw-opacity-0"
        >
          <p v-if="showCopiedMessage" class="tw-text-sm tw-text-green-600 tw-mt-2 tw-text-center">
            Address copied to clipboard!
          </p>
        </Transition>
      </div>
    </div>
  </template>
  
  <script setup>
  import { ref } from 'vue'
  import { Menu, MenuButton, MenuItem, MenuItems, TransitionRoot as Transition } from '@headlessui/vue'
  import { EllipsisVerticalIcon as DotsVerticalIcon, ClipboardIcon as ClipboardCopyIcon } from "@heroicons/vue/24/solid"
  
  let prop = defineProps({
    address : {
      type: String,
      required: true
    }
  })
  const walletAddress = ref(prop.address)
  const showCopiedMessage = ref(false)
  
  const copyToClipboard = () => {
    navigator.clipboard.writeText(walletAddress.value)
    showCopiedMessage.value = true
    setTimeout(() => {
      showCopiedMessage.value = false
    }, 2000)
  }
  </script>