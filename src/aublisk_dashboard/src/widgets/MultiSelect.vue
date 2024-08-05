<template>
    <Combobox @update:model-value="handleUpdate" as="div">
        <div class="tw-flex tw-flex-wrap tw-space-x-2">
            <button
            v-for="(skill, index) in selected"
            @click="removeSelected(index)"
                class="tw-bg-[#5BBB7B] hover:tw-bg-[#1F4B3F] tw-text-white tw-px-4 tw-py-1 tw-rounded-full tw-text-sm">{{ skill }}</button>
        </div>
        <!-- <ComboboxLabel class="tw-block tw-text-sm tw-font-medium tw-leading-6 tw-text-gray-900">Assigned to</ComboboxLabel> -->
        <div class="tw-relative tw-mt-2">
            <ComboboxInput @keyup.enter="handleEnter"
                class="tw-w-full tw-rounded-md tw-border-0 tw-bg-white tw-py-3 tw-pl-3 tw-pr-10 tw-text-gray-900 tw-shadow-sm tw-ring-1 tw-ring-inset tw-ring-gray-300 tw-focus:ring-2 tw-focus:ring-inset tw-focus:ring-indigo-600 tw-sm:text-sm tw-sm:leading-6"
                @change="query = $event.target.value" @blur="query = ''" :display-value="(person) => person?.name" />
            <ComboboxButton
                class="tw-absolute tw-inset-y-0 tw-right-0 tw-flex tw-items-center tw-rounded-r-md tw-px-2 tw-focus:outline-none">
                <ChevronUpDownIcon class="tw-h-5 tw-w-5 tw-text-gray-400" aria-hidden="true" />
            </ComboboxButton>
            <ComboboxOptions v-if="filteredPeople.length > 0"
                class="tw-absolute tw-z-10 tw-mt-1 tw-max-h-60 tw-w-full tw-overflow-auto tw-rounded-md tw-bg-white tw-py-1 tw-text-base tw-shadow-lg tw-ring-1 tw-ring-black tw-ring-opacity-5 tw-focus:outline-none tw-sm:text-sm">
                <ComboboxOption v-for="(person, idx) in filteredPeople" :key="idx" :value="person" as="template"
                    v-slot="{ active, selected }">
                    <li
                        :class="['tw-relative tw-cursor-default tw-select-none tw-py-2 tw-pl-3 tw-pr-9', active ? 'tw-bg-indigo-600 tw-text-white' : 'tw-text-gray-900']">
                        <span :class="['tw-block tw-truncate', selected && 'tw-font-semibold']">
                            {{ person }}
                        </span>
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
<script setup>
import { ref, computed } from 'vue'
import {
    Combobox,
    ComboboxInput,
    ComboboxButton,
    ComboboxOptions,
    ComboboxOption,
    TransitionRoot,
} from '@headlessui/vue'
import { CheckIcon, ChevronUpDownIcon } from '@heroicons/vue/20/solid'

let prop = defineProps({
  modelValue: {
    type: Boolean,
    required: true
  },

  options: {
    type: Array,
    required: true
  }
})

const emit = defineEmits(['update:modelValue'])


let selected = ref(prop.modelValue)
let query = ref('')
let filteredPeople = computed(() =>
    query.value === ''
        ? prop.options || []
        : prop.options.filter((person) =>
            person
                .toLowerCase()
                .replace(/\s+/g, '')
                .includes(query.value.toLowerCase().replace(/\s+/g, '')) || []
        )
)

const handleUpdate = (data) => {
    selected.value = [...selected.value, data]
    emit('update:modelValue', selected)
}

const handleEnter = () => {
    selected.value = [...selected.value, query.value]
    emit('update:modelValue', selected)
    // Add your logic here
}

const removeSelected = (index) => {
    selected.value.splice(index, 1)
    emit('update:modelValue', selected)
}
</script>