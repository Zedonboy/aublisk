<script setup lang="ts">
import MobileNav from './MobileNav.vue';
</script>

<template>
    <div class="row pb40">
        <div class="col-lg-12">
            <div class="dashboard_navigationbar d-block d-lg-none">
                <MobileNav />
            </div>
        </div>
        <div class="col-lg-12 tw-flex tw-justify-between">
            <div class="dashboard_title_area">
                <h2 class="tw-font-bold tw-text-3xl">Wallet</h2>
                <p class="text">Gm Anonymous</p>
            </div>
            <div class="tw-space-x-3">
                <button type="button"
                    class="tw-inline-flex tw-items-center tw-gap-x-1.5 tw-rounded-md tw-bg-black tw-px-3 tw-py-2 tw-text-sm tw-font-semibold tw-text-white tw-shadow-sm hover:tw-bg-[#5BBB7B] focus-visible:tw-outline focus-visible:tw-outline-2 focus-visible:tw-outline-offset-2 focus-visible:tw-outline-indigo-600">
                    <svg class="tw--ml-0.5 tw-h-5 tw-w-5" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true"
                        xmlns="http://www.w3.org/2000/svg" stroke-width="1.5" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round"
                            d="M3 16.5v2.25A2.25 2.25 0 0 0 5.25 21h13.5A2.25 2.25 0 0 0 21 18.75V16.5M16.5 12 12 16.5m0 0L7.5 12m4.5 4.5V3" />
                    </svg>


                    Deposit
                </button>
                <button type="button"
                    class="tw-inline-flex tw-items-center tw-gap-x-1.5 tw-rounded-md tw-bg-black tw-px-3 tw-py-2 tw-text-sm tw-font-semibold tw-text-white tw-shadow-sm hover:tw-bg-[#5BBB7B] focus-visible:tw-outline focus-visible:tw-outline-2 focus-visible:tw-outline-offset-2 focus-visible:tw-outline-indigo-600">
                    <svg class="tw--ml-0.5 tw-h-5 tw-w-5" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true"
                        xmlns="http://www.w3.org/2000/svg" stroke-width="1.5" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round"
                            d="M3 16.5v2.25A2.25 2.25 0 0 0 5.25 21h13.5A2.25 2.25 0 0 0 21 18.75V16.5m-13.5-9L12 3m0 0 4.5 4.5M12 3v13.5" />
                    </svg>

                    Withdraw
                </button>

            </div>
        </div>
    </div>
    <div class="row">
        <div class="col-sm-6 col-xxl-3">
            <div class="d-flex align-items-center justify-content-between statistics_funfact">
                <div class="details">
                    <div class="fz15">Services Offered</div>
                    <div class="title">25</div>
                    <div class="text fz14"><span class="text-thm">10</span> New Offered</div>
                </div>
                <div class="icon text-center"><i class="flaticon-contract"></i></div>
            </div>
        </div>


    </div>
    <div class="flex">
        <div class="tw-container tw-mx-auto tw-p-6 tw-bg-gray-100">
            <div class="tw-bg-white tw-rounded-lg tw-shadow-md tw-overflow-hidden">
                <table class="tw-w-full">
                    <thead>
                        <tr class="tw-bg-gray-50 tw-text-gray-600 tw-uppercase tw-text-sm tw-leading-normal">
                            <th class="tw-py-3 tw-px-6 tw-text-left">Asset</th>
                            <th class="tw-py-3 tw-px-6 tw-text-right">Balance</th>
                            <th class="tw-py-3 tw-px-6 tw-text-right">Value</th>
                            <th class="tw-py-3 tw-px-6 tw-text-center">Receive/Send</th>
                        </tr>
                    </thead>
                    <tbody class="tw-text-gray-600 tw-text-sm tw-font-light">
                        <!-- Bitcoin -->
                        <tr class="tw-border-b tw-border-gray-200 hover:tw-bg-gray-50">
                            <td class="tw-py-3 tw-px-6 tw-text-left tw-whitespace-nowrap">
                                <div class="tw-flex tw-items-center">
                                    <div class="tw-mr-2">
                                        <img class="tw-w-6 tw-h-6"
                                            src="https://cryptologos.cc/logos/bitcoin-btc-logo.png" alt="Bitcoin">
                                    </div>
                                    <span class="tw-font-medium">Bitcoin</span>
                                </div>
                            </td>
                            <td class="tw-py-3 tw-px-6 tw-text-right">
                                <span class="tw-font-medium">2.5369 BTC</span>
                            </td>
                            <td class="tw-py-3 tw-px-6 tw-text-right">
                                <span class="tw-font-medium">$98,562.34</span>
                            </td>
                            <td class="tw-py-3 tw-px-6 tw-text-center">
                                <div class="tw-flex tw-justify-center tw-items-center tw-space-x-2">
                                    <button
                                        class="tw-p-1 tw-rounded-full tw-bg-green-500 tw-text-white hover:tw-bg-green-600 focus:tw-outline-none focus:tw-ring-2 focus:tw-ring-offset-2 focus:tw-ring-green-500">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="tw-h-5 tw-w-5"
                                            viewBox="0 0 20 20" fill="currentColor">
                                            <path fill-rule="evenodd"
                                                d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z"
                                                clip-rule="evenodd" />
                                        </svg>
                                    </button>
                                    <button
                                        class="tw-p-1 tw-rounded-full tw-bg-blue-500 tw-text-white hover:tw-bg-blue-600 focus:tw-outline-none focus:tw-ring-2 focus:tw-ring-offset-2 focus:tw-ring-blue-500">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="tw-h-5 tw-w-5"
                                            viewBox="0 0 20 20" fill="currentColor">
                                            <path fill-rule="evenodd"
                                                d="M3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z"
                                                clip-rule="evenodd" />
                                        </svg>
                                    </button>
                                </div>
                            </td>
                        </tr>
                        <!-- Ethereum -->
                        <tr class="tw-border-b tw-border-gray-200 hover:tw-bg-gray-50">
                            <td class="tw-py-3 tw-px-6 tw-text-left tw-whitespace-nowrap">
                                <div class="tw-flex tw-items-center">
                                    <div class="tw-mr-2">
                                        <img class="tw-w-6 tw-h-6"
                                            src="https://cryptologos.cc/logos/ethereum-eth-logo.png" alt="Ethereum">
                                    </div>
                                    <span class="tw-font-medium">Ethereum</span>
                                </div>
                            </td>
                            <td class="tw-py-3 tw-px-6 tw-text-right">
                                <span class="tw-font-medium">18.7523 ETH</span>
                            </td>
                            <td class="tw-py-3 tw-px-6 tw-text-right">
                                <span class="tw-font-medium">$45,678.90</span>
                            </td>
                            <td class="tw-py-3 tw-px-6 tw-text-center">
                                <div class="tw-flex tw-justify-center tw-items-center tw-space-x-2">
                                    <button
                                        class="tw-p-1 tw-rounded-full tw-bg-green-500 tw-text-white hover:tw-bg-green-600 focus:tw-outline-none focus:tw-ring-2 focus:tw-ring-offset-2 focus:tw-ring-green-500">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="tw-h-5 tw-w-5"
                                            viewBox="0 0 20 20" fill="currentColor">
                                            <path fill-rule="evenodd"
                                                d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z"
                                                clip-rule="evenodd" />
                                        </svg>
                                    </button>
                                    <button
                                        class="tw-p-1 tw-rounded-full tw-bg-blue-500 tw-text-white hover:tw-bg-blue-600 focus:tw-outline-none focus:tw-ring-2 focus:tw-ring-offset-2 focus:tw-ring-blue-500">
                                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 20 20"
                                            stroke-width="1.5" stroke="currentColor" class="tw-h-5 tw-w-5">
                                            <path stroke-linecap="round" stroke-linejoin="round"
                                                d="M7.5 21 3 16.5m0 0L7.5 12M3 16.5h13.5m0-13.5L21 7.5m0 0L16.5 12M21 7.5H7.5" />
                                        </svg>


                                    </button>
                                </div>
                            </td>
                        </tr>
                        <!-- Add more rows for other cryptocurrencies with the same structure -->
                    </tbody>
                </table>
            </div>
        </div>
    </div>
</template>