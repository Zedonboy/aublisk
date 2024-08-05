<template>
  <Combobox as="div" v-model="selectedData" @update:modelValue="handleUpdate">
    <div class="tw-relative tw-mt-2">
      <div class="tw-w-full tw-flex tw-items-center tw-space-x-1 tw-rounded-md tw-border-0 tw-bg-white tw-py-3 tw-pl-3 tw-pr-12 tw-text-gray-900 tw-shadow-sm tw-ring-1 tw-ring-inset tw-ring-gray-300 tw-focus:ring-2 tw-focus:ring-inset tw-focus:ring-indigo-600 tw-sm:text-sm tw-sm:leading-6">
        <img v-if="selectedData?.[props.logoKey]" :src="selectedData?.[props.logoKey]" class="tw-h-5 tw-w-5 tw-text-gray-400" aria-hidden="true" />
        <ComboboxInput
        class="p-0 tw-w-full tw-border-0 focus:tw-outline-none focus:tw-ring-0"
        @change="query = $event.target.value"
        @blur="query = ''"
        :display-value="(data) => data?.[props.nameKey]"
      />
      </div>
     
      <ComboboxButton
        class="tw-absolute tw-inset-y-0 tw-right-0 tw-flex tw-items-center tw-rounded-r-md tw-px-2 tw-focus:outline-none">
        <ChevronUpDownIcon class="tw-h-5 tw-w-5 tw-text-gray-400" aria-hidden="true" />
      </ComboboxButton>
      <ComboboxOptions v-if="filteredData.length > 0"
        class="tw-absolute tw-z-10 tw-mt-1 tw-max-h-56 tw-w-full tw-overflow-auto tw-rounded-md tw-bg-white tw-py-1 tw-text-base tw-shadow-lg tw-ring-1 tw-ring-black tw-ring-opacity-5 tw-focus:outline-none tw-sm:text-sm">
        <ComboboxOption v-for="data in filteredData" :key="data[props.valueKey]" :value="data" as="template"
          v-slot="{ active, selected }">
          <li
            :class="['tw-relative tw-cursor-default tw-select-none tw-py-2 tw-pl-3 tw-pr-9', active ? 'tw-bg-indigo-600 tw-text-white' : 'tw-text-gray-900']">
            <div class="tw-flex tw-items-center">
              <img v-if="data?.[props.logoKey]" :src="data?.[props.logoKey]" alt="" class="tw-h-6 tw-w-6 tw-flex-shrink-0 tw-rounded-full" />
              <span :class="['tw-ml-3 tw-truncate', selected && 'tw-font-semibold']">
                {{ data?.[props.nameKey] }}
              </span>
            </div>
            <span v-if="selected"
              :class="['tw-absolute tw-inset-y-0 tw-right-0 tw-flex tw-items-center tw-pr-4', active ? 'tw-text-white' : 'tw-text-indigo-600']">
              <CheckIcon class="tw-h-5 tw-w-5" aria-hidden="true" />
            </span>
          </li>
        </ComboboxOption>
      </ComboboxOptions>
    </div>
  </Combobox>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { CheckIcon, ChevronUpDownIcon } from '@heroicons/vue/20/solid'
import {
  Combobox,
  ComboboxButton,
  ComboboxInput,
  ComboboxOption,
  ComboboxOptions,
} from '@headlessui/vue'

interface Option {
  name: string;
  value: string;
  logo?: string;
}

const props = defineProps<{
  options: Option[];
  modelValue: Option | null;
  valueKey : string
  nameKey : string,
  logoKey: string
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: Option | null): void;
}>()

const query = ref('')
const selectedData = ref(props.modelValue)

const filteredData = computed(() =>
  query.value === ''
    ? props.options
    : props.options.filter((data) =>
    //@ts-ignore
        data[props.nameKey].toLowerCase().includes(query.value.toLowerCase())
      )
)

const handleUpdate = (value: Option | null) => {
  selectedData.value = value
  emit('update:modelValue', value)
  query.value = ''
}
</script>