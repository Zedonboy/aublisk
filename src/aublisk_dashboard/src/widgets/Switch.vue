<template>
  <Switch :modelValue="modelValue"
  @update:model-value="$emit('update:modelValue', $event)"
    :class="[modelValue ? 'tw-bg-indigo-600' : 'tw-bg-gray-200', 'tw-relative tw-inline-flex tw-h-6 tw-w-11 tw-flex-shrink-0 tw-cursor-pointer tw-rounded-full tw-border-2 tw-border-transparent tw-transition-colors tw-duration-200 tw-ease-in-out tw-focus:outline-none tw-focus:ring-2 tw-focus:ring-indigo-600 tw-focus:ring-offset-2']">
    <span class="tw-sr-only">Use setting</span>
    <span aria-hidden="true"
      :class="[modelValue ? 'tw-translate-x-5' : 'tw-translate-x-0', 'tw-pointer-events-none tw-inline-block tw-h-5 tw-w-5 tw-transform tw-rounded-full tw-bg-white tw-shadow tw-ring-0 tw-transition tw-duration-200 tw-ease-in-out']" />
  </Switch>
</template>

<script setup>
import { ref } from 'vue'
import { Switch } from '@headlessui/vue'

defineProps({
  modelValue: {
    type: Boolean,
    required: true
  }
})

defineEmits(['update:modelValue'])


</script>