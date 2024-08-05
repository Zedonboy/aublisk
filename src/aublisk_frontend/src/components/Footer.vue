<template>
    <footer class="tw-bg-white">
        <div class="tw-mx-auto tw-max-w-7xl tw-overflow-hidden tw-px-6 tw-py-20 sm:tw-py-24 lg:tw-px-8">
            <nav class="tw--mb-6 tw-columns-2 sm:tw-flex sm:tw-justify-center sm:tw-space-x-12" aria-label="Footer">
                <div v-for="item in navigation.main" :key="item.name" class="tw-pb-6">
                    <a :href="item.href" class="tw-text-sm tw-leading-6 tw-text-gray-600 hover:tw-text-gray-900">{{
                        item.name }}</a>
                </div>
            </nav>
            <div class="tw-mt-10 tw-flex tw-justify-center tw-space-x-10">
                <a v-for="item in navigation.social" :key="item.name" :href="item.href"
                    class="tw-text-gray-400 hover:tw-text-gray-500">
                    <span class="tw-sr-only">{{ item.name }}</span>
                    <div v-html="item.icon" class="tw-h-6 tw-w-6" aria-hidden="true" />
                </a>
            </div>
            <p class="tw-mt-10 tw-text-center tw-text-xs tw-leading-5 tw-text-gray-500">&copy; 2024 Aublisk, Inc.
                All rights reserved.</p>
        </div>
    </footer>
</template>

<script setup>
import { defineComponent, h } from 'vue'

const navigation = {
    main: [
        { name: 'About', href: '#' },
        { name: 'Blog', href: '#' },
        { name: 'Jobs', href: '#' },
        { name: 'Press', href: '#' },
        { name: 'Accessibility', href: '#' },
        { name: 'Partners', href: '#' },
    ],
    social: [
        {
            name: 'LinkedIn',
            href: 'https://www.linkedin.com/company/aublisk',
            icon: `<span class="[&>svg]:h-5 [&>svg]:w-5">
  <svg
    xmlns="http://www.w3.org/2000/svg"
    fill="currentColor"
    viewBox="0 0 448 512">
    <!--!Font Awesome Free 6.5.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc. -->
    <path
      d="M100.3 448H7.4V148.9h92.9zM53.8 108.1C24.1 108.1 0 83.5 0 53.8a53.8 53.8 0 0 1 107.6 0c0 29.7-24.1 54.3-53.8 54.3zM447.9 448h-92.7V302.4c0-34.7-.7-79.2-48.3-79.2-48.3 0-55.7 37.7-55.7 76.7V448h-92.8V148.9h89.1v40.8h1.3c12.4-23.5 42.7-48.3 87.9-48.3 94 0 111.3 61.9 111.3 142.3V448z" />
  </svg>
</span>`
        },

        {
            name: 'Twitter',
            href: 'https://x.com/aublisk',
            icon: `<span class="[&>svg]:h-7 [&>svg]:w-7 [&>svg]:fill-black">
  <svg
    xmlns="http://www.w3.org/2000/svg"
    fill="currentColor"
    viewBox="0 0 512 512">
    <!--!Font Awesome Free 6.5.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc. -->
    <path
      d="M389.2 48h70.6L305.6 224.2 487 464H345L233.7 318.6 106.5 464H35.8L200.7 275.5 26.8 48H172.4L272.9 180.9 389.2 48zM364.4 421.8h39.1L151.1 88h-42L364.4 421.8z" />
  </svg>
</span>
`
        },

        {
            name: 'Discord',
            href: 'https://discord.gg/Pgjwcw93WZ',
            icon: `<span class="tw-[&>svg]:h-7 tw-[&>svg]:w-7 tw-[&>svg]:fill-[#7289da]">
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 512">
    <!--!Font Awesome Free 6.5.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc. -->
    <path
      d="M524.5 69.8a1.5 1.5 0 0 0 -.8-.7A485.1 485.1 0 0 0 404.1 32a1.8 1.8 0 0 0 -1.9 .9 337.5 337.5 0 0 0 -14.9 30.6 447.8 447.8 0 0 0 -134.4 0 309.5 309.5 0 0 0 -15.1-30.6 1.9 1.9 0 0 0 -1.9-.9A483.7 483.7 0 0 0 116.1 69.1a1.7 1.7 0 0 0 -.8 .7C39.1 183.7 18.2 294.7 28.4 404.4a2 2 0 0 0 .8 1.4A487.7 487.7 0 0 0 176 479.9a1.9 1.9 0 0 0 2.1-.7A348.2 348.2 0 0 0 208.1 430.4a1.9 1.9 0 0 0 -1-2.6 321.2 321.2 0 0 1 -45.9-21.9 1.9 1.9 0 0 1 -.2-3.1c3.1-2.3 6.2-4.7 9.1-7.1a1.8 1.8 0 0 1 1.9-.3c96.2 43.9 200.4 43.9 295.5 0a1.8 1.8 0 0 1 1.9 .2c2.9 2.4 6 4.9 9.1 7.2a1.9 1.9 0 0 1 -.2 3.1 301.4 301.4 0 0 1 -45.9 21.8 1.9 1.9 0 0 0 -1 2.6 391.1 391.1 0 0 0 30 48.8 1.9 1.9 0 0 0 2.1 .7A486 486 0 0 0 610.7 405.7a1.9 1.9 0 0 0 .8-1.4C623.7 277.6 590.9 167.5 524.5 69.8zM222.5 337.6c-29 0-52.8-26.6-52.8-59.2S193.1 219.1 222.5 219.1c29.7 0 53.3 26.8 52.8 59.2C275.3 311 251.9 337.6 222.5 337.6zm195.4 0c-29 0-52.8-26.6-52.8-59.2S388.4 219.1 417.9 219.1c29.7 0 53.3 26.8 52.8 59.2C470.7 311 447.5 337.6 417.9 337.6z" />
  </svg>
</span>`
        }
    ]
}
</script>