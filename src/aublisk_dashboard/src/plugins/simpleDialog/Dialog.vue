<template>
   <TransitionRoot as="template" :show="isOpen">
  <Dialog class="tw-relative tw-z-10" @close="isOpen = false">
    <TransitionChild as="template" enter="tw-ease-out tw-duration-300" enter-from="tw-opacity-0" enter-to="tw-opacity-100" leave="tw-ease-in tw-duration-200" leave-from="tw-opacity-100" leave-to="tw-opacity-0">
      <div class="tw-fixed tw-inset-0 tw-bg-gray-500 tw-bg-opacity-75 tw-transition-opacity" />
    </TransitionChild>

    <div class="tw-fixed tw-inset-0 tw-z-10 tw-w-screen tw-overflow-y-auto">
      <div class="tw-flex tw-min-h-full tw-items-end tw-justify-center tw-p-4 tw-text-center sm:tw-items-center sm:tw-p-0">
        <TransitionChild as="template" enter="tw-ease-out tw-duration-300" enter-from="tw-opacity-0 tw-translate-y-4 sm:tw-translate-y-0 sm:tw-scale-95" enter-to="tw-opacity-100 tw-translate-y-0 sm:tw-scale-100" leave="tw-ease-in tw-duration-200" leave-from="tw-opacity-100 tw-translate-y-0 sm:tw-scale-100" leave-to="tw-opacity-0 tw-translate-y-4 sm:tw-translate-y-0 sm:tw-scale-95">
          <DialogPanel class="tw-relative tw-transform tw-overflow-hidden tw-rounded-lg tw-bg-white tw-px-4 tw-pb-4 tw-pt-5 tw-text-left tw-shadow-xl tw-transition-all sm:tw-my-8 sm:tw-w-full sm:tw-max-w-lg sm:tw-p-6">
            <div class="sm:tw-flex sm:tw-items-start">
              <div class="tw-mx-auto tw-flex tw-h-12 tw-w-12 tw-flex-shrink-0 tw-items-center tw-justify-center tw-rounded-full tw-bg-red-100 sm:tw-mx-0 sm:tw-h-10 sm:tw-w-10">
                <ExclamationTriangleIcon class="tw-h-6 tw-w-6 tw-text-red-600" aria-hidden="true" />
              </div>
              <div class="tw-mt-3 tw-text-center sm:tw-ml-4 sm:tw-mt-0 sm:tw-text-left">
                <DialogTitle as="h3" class="tw-text-base tw-font-semibold tw-leading-6 tw-text-gray-900">{{ title }}</DialogTitle>
                <div class="tw-mt-2">
                  <p class="tw-text-sm tw-text-gray-500">{{ message }}</p>
                </div>
              </div>
            </div>
            <div class="tw-mt-5 sm:tw-mt-4 sm:tw-flex sm:tw-flex-row-reverse">
              <button type="button" class="tw-inline-flex tw-w-full tw-justify-center tw-rounded-md tw-bg-red-600 tw-px-3 tw-py-2 tw-text-sm tw-font-semibold tw-text-white tw-shadow-sm hover:tw-bg-red-500 sm:tw-ml-3 sm:tw-w-auto" @click="confirm">{{ confirmText }}</button>
              <button type="button" class="tw-mt-3 tw-inline-flex tw-w-full tw-justify-center tw-rounded-md tw-bg-white tw-px-3 tw-py-2 tw-text-sm tw-font-semibold tw-text-gray-900 tw-shadow-sm tw-ring-1 tw-ring-inset tw-ring-gray-300 hover:tw-bg-gray-50 sm:tw-mt-0 sm:tw-w-auto" @click="cancel">{{ cancelText }}</button>
            </div>
          </DialogPanel>
        </TransitionChild>
      </div>
    </div>
  </Dialog>
</TransitionRoot>
  </template>
  
  <script setup>
  import { ref } from 'vue'
  import { Dialog, DialogPanel, DialogTitle, TransitionChild, TransitionRoot } from '@headlessui/vue'
  import { ExclamationTriangleIcon } from '@heroicons/vue/24/outline'
  
  const props = defineProps({
  title: {
    type: String,
    default: 'Confirmation'
  },
  message: {
    type: String,
    default: 'Are you sure you want to perform this action?'
  },
  confirmText: {
    type: String,
    default: 'Confirm'
  },
  cancelText: {
    type: String,
    default: 'Cancel'
  }
})

const emit = defineEmits(['confirm', 'cancel', 'close'])

const isOpen = ref(true)

const closeDialog = () => {
  isOpen.value = false
  emit('close')
}

const confirm = async () => {
  emit('confirm')
  closeDialog()
}

const cancel = async () => {
  emit('cancel')
  closeDialog()
}

</script>