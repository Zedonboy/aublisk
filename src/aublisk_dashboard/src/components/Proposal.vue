<template>
    <main class="tw-profile-page">
      <section class="tw-relative tw-block tw-h-500-px">
        <div class="tw-absolute tw-top-0 tw-w-full tw-h-full tw-bg-center tw-bg-cover">
          <span id="blackOverlay" class="tw-w-full tw-h-full tw-absolute tw-opacity-50 tw-bg-black"></span>
        </div>
        <div class="tw-top-auto tw-bottom-0 tw-left-0 tw-right-0 tw-w-full tw-absolute tw-pointer-events-none tw-overflow-hidden tw-h-70-px" style="transform: translateZ(0px)">
          <svg class="tw-absolute tw-bottom-0 tw-overflow-hidden" xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="none" version="1.1" viewBox="0 0 2560 100" x="0" y="0">
            <polygon class="tw-text-blueGray-200 tw-fill-current" points="2560 0 2560 100 0 100"></polygon>
          </svg>
        </div>
      </section>
      <section class="tw-relative tw-py-16 tw-bg-blueGray-200">
        <div class="tw-container tw-mx-auto tw-px-4">
          <div class="tw-relative tw-flex tw-flex-col tw-min-w-0 tw-break-words tw-bg-white tw-w-full tw-mb-6 tw-shadow-xl tw-rounded-lg tw--mt-64">
            <div class="tw-px-6">
              <div class="tw-flex tw-flex-wrap tw-justify-center">
                <div class="tw-w-full lg:tw-w-3/12 tw-px-4 lg:tw-order-2 tw-flex tw-justify-center">
                  <div class="tw-relative">
                    <img :alt="profile.name" :src="profile.avatar" class="tw-shadow-xl tw-max-w-48 tw-h-auto tw-rounded-full tw-align-middle tw-border-none -tw-mt-16">
                  </div>
                </div>
                <div class="tw-w-full lg:tw-w-4/12 tw-px-4 lg:tw-order-3 lg:tw-text-right lg:tw-self-center">
                  <div class="tw-py-6 tw-px-3 tw-mt-32 sm:tw-mt-0">
                    <button @click="connect" class="ud-btn btn-dark default-box-shadow2" type="button">
                      Assign
                    </button>
                  </div>
                </div>
                <div class="tw-w-full lg:tw-w-4/12 tw-px-4 lg:tw-order-1">
                  <div class="tw-flex tw-justify-center tw-py-4 lg:tw-pt-4 tw-pt-8">
                    <div class="tw-mr-4 tw-p-3 tw-text-center">
                      <span class="tw-text-xl tw-font-bold tw-block tw-uppercase tw-tracking-wide tw-text-blueGray-600">{{ profile.friends }}</span><span class="tw-text-sm tw-text-blueGray-400">Friends</span>
                    </div>
                    <div class="tw-mr-4 tw-p-3 tw-text-center">
                      <span class="tw-text-xl tw-font-bold tw-block tw-uppercase tw-tracking-wide tw-text-blueGray-600">{{ profile.photos }}</span><span class="tw-text-sm tw-text-blueGray-400">Photos</span>
                    </div>
                    <div class="lg:tw-mr-4 tw-p-3 tw-text-center">
                      <span class="tw-text-xl tw-font-bold tw-block tw-uppercase tw-tracking-wide tw-text-blueGray-600">{{ profile.comments }}</span><span class="tw-text-sm tw-text-blueGray-400">Comments</span>
                    </div>
                  </div>
                </div>
              </div>
              <div class="tw-text-center tw-mt-6">
                <h3 class="tw-text-4xl tw-font-semibold tw-leading-normal tw-text-blueGray-700 tw-mb-2">
                  {{ profile.name }}
                </h3>
                <div class="tw-text-sm tw-leading-normal tw-mt-0 tw-mb-2 tw-text-blueGray-400 tw-font-bold tw-uppercase">
                  <LocationMarkerIcon class="tw-inline-block tw-w-5 tw-h-5 tw-mr-2 tw-text-blueGray-400" />
                  {{ profile.location }}
                </div>
                <div class="tw-mb-2 tw-text-blueGray-600 tw-mt-10">
                  <BriefcaseIcon class="tw-inline-block tw-w-5 tw-h-5 tw-mr-2 tw-text-blueGray-400" />{{ profile.job }}
                </div>
                <div class="tw-mb-2 tw-text-blueGray-600">
                  <AcademicCapIcon class="tw-inline-block tw-w-5 tw-h-5 tw-mr-2 tw-text-blueGray-400" />{{ profile.education }}
                </div>
              </div>
              <div class="tw-mt-10 tw-py-10 tw-border-t tw-border-blueGray-200 tw-text-center">
                <div class="tw-flex tw-flex-wrap tw-justify-center">
                  <div class="tw-w-full lg:tw-w-9/12 tw-px-4">
                    <p class="tw-mb-4 tw-text-lg tw-leading-relaxed tw-text-blueGray-700">
                      {{ profile.about }}
                    </p>
                    <a href="" class="tw-font-normal tw-text-pink-500">Show more</a>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
        
      </section>
    </main>
  </template>
  
  <script setup>
  import { ref } from 'vue'
  import { MapPinIcon as LocationMarkerIcon, BriefcaseIcon, AcademicCapIcon } from '@heroicons/vue/24/solid'
  
  const backgroundImage = ref('https://images.unsplash.com/photo-1499336315816-097655dcfbda?ixlib=rb-1.2.1&amp;ixid=eyJhcHBfaWQiOjEyMDd9&amp;auto=format&amp;fit=crop&amp;w=2710&amp;q=80')
  
  const profile = ref({
    name: 'Jenna Stones',
    avatar: 'https://demos.creative-tim.com/notus-js/assets/img/team-2-800x800.jpg',
    location: 'Los Angeles, California',
    job: 'Solution Manager - Creative Tim Officer',
    education: 'University of Computer Science',
    friends: 22,
    photos: 10,
    comments: 89,
    about: 'An artist of considerable range, Jenna the name taken by Melbourne-raised, Brooklyn-based Nick Murphy writes, performs and records all of his own music, giving it a warm, intimate feel with a solid groove structure. An artist of considerable range.'
  })
  
  const connect = () => {
    // Implement connect functionality
    console.log('Connect button clicked')
  }
  </script>jgu