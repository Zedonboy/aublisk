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
                <h2 class="tw-font-bold tw-text-3xl">Dao</h2>
                <p class="text">Staking and Rewards</p>
            </div>
            <div class="tw-space-x-3">

                <button type="button"
                    class="tw-inline-flex tw-items-center tw-gap-x-1.5 tw-rounded-md tw-bg-black tw-px-3 tw-py-2 tw-text-sm tw-font-semibold tw-text-white tw-shadow-sm hover:tw-bg-[#5BBB7B] focus-visible:tw-outline focus-visible:tw-outline-2 focus-visible:tw-outline-offset-2 focus-visible:tw-outline-indigo-600">
                    <svg class="tw--ml-0.5 tw-h-5 tw-w-5" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true"
                        xmlns="http://www.w3.org/2000/svg" stroke-width="1.5" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round"
                            d="M3 16.5v2.25A2.25 2.25 0 0 0 5.25 21h13.5A2.25 2.25 0 0 0 21 18.75V16.5M16.5 12 12 16.5m0 0L7.5 12m4.5 4.5V3" />
                    </svg>


                    Stake Token
                </button>

                <button type="button"
                    class="tw-inline-flex tw-items-center tw-gap-x-1.5 tw-rounded-md tw-bg-black tw-px-3 tw-py-2 tw-text-sm tw-font-semibold tw-text-white tw-shadow-sm hover:tw-bg-[#5BBB7B] focus-visible:tw-outline focus-visible:tw-outline-2 focus-visible:tw-outline-offset-2 focus-visible:tw-outline-indigo-600">
                    <svg class="tw--ml-0.5 tw-h-5 tw-w-5" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true"
                        xmlns="http://www.w3.org/2000/svg" stroke-width="1.5" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round"
                            d="M3 16.5v2.25A2.25 2.25 0 0 0 5.25 21h13.5A2.25 2.25 0 0 0 21 18.75V16.5m-13.5-9L12 3m0 0 4.5 4.5M12 3v13.5" />
                    </svg>

                    Dissolve Token
                </button>

            </div>
        </div>
    </div>
    
    
          <div class="row">
            <div class="col-xl-12">
              <div class="ps-widget bgc-white bdrs4 p30 mb30 overflow-hidden position-relative">
                <div class="packages_table table-responsive">
                  <table class="table-style3 table at-savesearch">
                    <thead class="t-head">
                      <tr>
                        <th scope="col">Invoice ID</th>
                        <th scope="col">Purchase Date</th>
                        <th scope="col">Amount</th>
                        <th scope="col">Payment Status</th>
                        <th scope="col">Action</th>
                      </tr>
                    </thead>
                    <tbody class="t-body">
                      <tr>
                        <th scope="row">
                          <div>#99 <span class="ms-3">App Services</span></div>
                        </th>
                        <td class="vam">April 9, 2023</td>
                        <td class="vam">$1.200</td>
                        <td class="vam"><span class="pending-style style1">In Progress</span></td>
                        <td class="vam">
                          <a href="" class="table-action fz15 fw500 text-thm2" data-bs-toggle="tooltip" data-bs-placement="top" title="View"><span class="flaticon-website me-2 vam"></span> View</a>
                        </td>
                      </tr>
                      <tr>
                        <th scope="row">
                          <div>#99 <span class="ms-3">App Services</span></div>
                        </th>
                        <td class="vam">April 9, 2023</td>
                        <td class="vam">$1.200</td>
                        <td class="vam"><span class="pending-style style2">Completed</span></td>
                        <td class="vam">
                          <a href="" class="table-action fz15 fw500 text-thm2" data-bs-toggle="tooltip" data-bs-placement="top" title="View"><span class="flaticon-website me-2 vam"></span> View</a>
                        </td>
                      </tr>
                      <tr>
                        <th class="active">
                          <div>#99 <span class="ms-3">App Services</span></div>
                        </th>
                        <td class="vam">April 9, 2023</td>
                        <td class="vam">$1.200</td>
                        <td class="vam"><span class="pending-style style2">Completed</span></td>
                        <td class="vam">
                          <a href="" class="table-action fz15 fw500 text-thm2" data-bs-toggle="tooltip" data-bs-placement="top" title="View"><span class="flaticon-website me-2 vam"></span> View</a>
                        </td>
                      </tr>
                      <tr>
                        <th scope="row">
                          <div>#99 <span class="ms-3">App Services</span></div>
                        </th>
                        <td class="vam">April 9, 2023</td>
                        <td class="vam">$1.200</td>
                        <td class="vam"><span class="pending-style style3">Cancel</span></td>
                        <td class="vam">
                          <a href="" class="table-action fz15 fw500 text-thm2" data-bs-toggle="tooltip" data-bs-placement="top" title="View"><span class="flaticon-website me-2 vam"></span> View</a>
                        </td>
                      </tr>
                      <tr>
                        <th scope="row">
                          <div>#99 <span class="ms-3">App Services</span></div>
                        </th>
                        <td class="vam">April 9, 2023</td>
                        <td class="vam">$1.200</td>
                        <td class="vam"><span class="pending-style style3">Cancel</span></td>
                        <td class="vam">
                          <a href="" class="table-action fz15 fw500 text-thm2" data-bs-toggle="tooltip" data-bs-placement="top" title="View"><span class="flaticon-website me-2 vam"></span> View</a>
                        </td>
                      </tr>
                      <tr>
                        <th class="active">
                          <div>#99 <span class="ms-3">App Services</span></div>
                        </th>
                        <td class="vam">April 9, 2023</td>
                        <td class="vam">$1.200</td>
                        <td class="vam"><span class="pending-style style2">Completed</span></td>
                        <td class="vam">
                          <a href="" class="table-action fz15 fw500 text-thm2" data-bs-toggle="tooltip" data-bs-placement="top" title="View"><span class="flaticon-website me-2 vam"></span> View</a>
                        </td>
                      </tr>
                      <tr>
                        <th scope="row">
                          <div>#99 <span class="ms-3">App Services</span></div>
                        </th>
                        <td class="vam">April 9, 2023</td>
                        <td class="vam">$1.200</td>
                        <td class="vam"><span class="pending-style style2">Completed</span></td>
                        <td class="vam">
                          <a href="" class="table-action fz15 fw500 text-thm2" data-bs-toggle="tooltip" data-bs-placement="top" title="View"><span class="flaticon-website me-2 vam"></span> View</a>
                        </td>
                      </tr>
                      <tr>
                        <th scope="row">
                          <div>#99 <span class="ms-3">App Services</span></div>
                        </th>
                        <td class="vam">April 9, 2023</td>
                        <td class="vam">$1.200</td>
                        <td class="vam"><span class="pending-style style1">In Progress</span></td>
                        <td class="vam">
                          <a href="" class="table-action fz15 fw500 text-thm2" data-bs-toggle="tooltip" data-bs-placement="top" title="View"><span class="flaticon-website me-2 vam"></span> View</a>
                        </td>
                      </tr>
                      <tr>
                        <th scope="row">
                          <div>#99 <span class="ms-3">App Services</span></div>
                        </th>
                        <td class="vam">April 9, 2023</td>
                        <td class="vam">$1.200</td>
                        <td class="vam"><span class="pending-style style3">Cancel</span></td>
                        <td class="vam">
                          <a href="" class="table-action fz15 fw500 text-thm2" data-bs-toggle="tooltip" data-bs-placement="top" title="View"><span class="flaticon-website me-2 vam"></span> View</a>
                        </td>
                      </tr>
                    </tbody>
                  </table>
                  <div class="mbp_pagination text-center mt30">
                    <ul class="page_navigation">
                      <li class="page-item">
                        <a class="page-link" href="#"> <span class="fas fa-angle-left"></span></a>
                      </li>
                      <li class="page-item"><a class="page-link" href="#">1</a></li>
                      <li class="page-item active" aria-current="page">
                        <a class="page-link" href="#">2 <span class="sr-only">(current)</span></a>
                      </li>
                      <li class="page-item"><a class="page-link" href="#">3</a></li>
                      <li class="page-item"><a class="page-link" href="#">4</a></li>
                      <li class="page-item"><a class="page-link" href="#">5</a></li>
                      <li class="page-item"><a class="page-link" href="#">...</a></li>
                      <li class="page-item"><a class="page-link" href="#">20</a></li>
                      <li class="page-item">
                        <a class="page-link" href="#"><span class="fas fa-angle-right"></span></a>
                      </li>
                    </ul>
                    <p class="mt10 mb-0 pagination_page_count text-center">1 – 20 of 300+ property available</p>
                  </div>
                </div>
              </div>
            </div>
          </div>
</template>