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
                <h2 class="tw-font-bold tw-text-3xl">Profile</h2>
                <p class="text">Rent Skills here</p>
            </div>
           
        </div>
    </div>
    
    
    <div class="row">
            <div class="col-xl-12">
              <div class="ps-widget bgc-white bdrs4 p30 mb30 overflow-hidden position-relative">
                <div class="bdrb1 pb15 mb25">
                  <h5 class="list-title">Profile Details</h5>
                </div>
                <div class="col-xl-7">
                  <div class="profile-box d-sm-flex align-items-center mb30">
                    <div class="profile-img mb20-sm">
                      <img class="w-100 rounded-circle wa-xs" src="/images/team/fl-1.png" alt="">
                    </div>
                    <div class="profile-content ml20 ml0-xs">
                      <div class="d-flex align-items-center my-3">
                        <a href="" class="tag-delt text-thm2"><span class="flaticon-delete text-thm2"></span></a>
                        <a href="" class="upload-btn ml10">Upload Images</a>
                      </div>
                      <p class="text mb-0">Max file size is 1MB, Minimum dimension: 330x300 And Suitable files are .jpg & .png</p>
                    </div>
                  </div>
                </div>
                <div class="col-lg-7">
                  <form class="form-style1">
                    <div class="row">
                      <div class="col-sm-6">
                        <div class="mb20">
                          <label class="heading-color ff-heading fw500 mb10">Username</label>
                          <input type="text" class="form-control" placeholder="i will">
                        </div>
                      </div>
                      <div class="col-sm-6">
                        <div class="mb20">
                          <label class="heading-color ff-heading fw500 mb10">Email Address</label>
                          <input type="email" class="form-control" placeholder="i will">
                        </div>
                      </div>
                      <div class="col-sm-6">
                        <div class="mb20">
                          <label class="heading-color ff-heading fw500 mb10">Phone Number</label>
                          <input type="text" class="form-control" placeholder="i will">
                        </div>
                      </div>
                      <div class="col-sm-6">
                        <div class="mb20">
                          <label class="heading-color ff-heading fw500 mb10">Tagline</label>
                          <input type="text" class="form-control" placeholder="i will">
                        </div>
                      </div>
                      <div class="col-sm-6">
                        <div class="mb20">
                          <div class="form-style1">
                            <label class="heading-color ff-heading fw500 mb10">Hourly Rate</label>
                            <div class="bootselect-multiselect">
                              <select class="selectpicker">
                                <option>Select</option>
                                <option>$50</option>
                                <option>$60</option>
                                <option>$70</option>
                                <option>$80</option>
                                <option>$90</option>
                                <option>$100</option>
                              </select>
                            </div>
                          </div>
                        </div>
                      </div>
                      <div class="col-sm-6">
                        <div class="mb20">
                          <div class="form-style1">
                            <label class="heading-color ff-heading fw500 mb10">Gender</label>
                            <div class="bootselect-multiselect">
                              <select class="selectpicker">
                                <option>Select</option>
                                <option>Male</option>
                                <option>Female</option>
                                <option>Other</option>
                              </select>
                            </div>
                          </div>
                        </div>
                      </div>
                      <div class="col-sm-6">
                        <div class="mb20">
                          <div class="form-style1">
                            <label class="heading-color ff-heading fw500 mb10">Specialization</label>
                            <div class="bootselect-multiselect">
                              <select class="selectpicker">
                                <option>Select</option>
                                <option>Male</option>
                                <option>Female</option>
                                <option>Other</option>
                              </select>
                            </div>
                          </div>
                        </div>
                      </div>
                      <div class="col-sm-6">
                        <div class="mb20">
                          <div class="form-style1">
                            <label class="heading-color ff-heading fw500 mb10">Type</label>
                            <div class="bootselect-multiselect">
                              <select class="selectpicker">
                                <option>Select</option>
                                <option>Type One</option>
                                <option>Type Two</option>
                                <option>Type Three</option>
                              </select>
                            </div>
                          </div>
                        </div>
                      </div>
                      <div class="col-sm-6">
                        <div class="mb20">
                          <div class="form-style1">
                            <label class="heading-color ff-heading fw500 mb10">Country</label>
                            <div class="bootselect-multiselect">
                              <select class="selectpicker">
                                <option>Turkey</option>
                                <option>United Kingdom</option>
                                <option>United State</option>
                                <option>Ukraine</option>
                                <option>Uruguay</option>
                                <option>UK</option>
                                <option>Uzbekistan</option>
                              </select>
                            </div>
                          </div>
                        </div>
                      </div>
                      <div class="col-sm-6">
                        <div class="mb20">
                          <div class="form-style1">
                            <label class="heading-color ff-heading fw500 mb10">City</label>
                            <div class="bootselect-multiselect">
                              <select class="selectpicker">
                                <option data-tokens="California">California</option>
                                <option data-tokens="Chicago">Chicago</option>
                                <option data-tokens="LosAngeles">Los Angeles</option>
                                <option data-tokens="Manhattan">Manhattan</option>
                                <option data-tokens="NewJersey">New Jersey</option>
                                <option data-tokens="NewYork">New York</option>
                                <option data-tokens="SanDiego">San Diego</option>
                                <option data-tokens="SanFrancisco">San Francisco</option>
                                <option data-tokens="Texas">Texas</option>
                              </select>
                            </div>
                          </div>
                        </div>
                      </div>
                      <div class="col-sm-6">
                        <div class="mb20">
                          <div class="form-style1">
                            <label class="heading-color ff-heading fw500 mb10">Languages</label>
                            <div class="bootselect-multiselect">
                              <select class="selectpicker" multiple>
                                <option>English</option>
                                <option>Spanish</option>
                                <option>Greek</option>
                                <option>Tarkish</option>
                              </select>
                            </div>
                          </div>
                        </div>
                      </div>
                      <div class="col-sm-6">
                        <div class="mb20">
                          <div class="form-style1">
                            <label class="heading-color ff-heading fw500 mb10">Languages Level</label>
                            <div class="bootselect-multiselect">
                              <select class="selectpicker">
                                <option>Select</option>
                                <option>Fluent</option>
                                <option>Mid Level</option>
                                <option>Conversational</option>
                                <option>Other</option>
                              </select>
                            </div>
                          </div>
                        </div>
                      </div>
                      <div class="col-md-12">
                        <div class="mb10">
                          <label class="heading-color ff-heading fw500 mb10">Introduce Yourself</label>
                          <textarea cols="30" rows="6" placeholder="Description"></textarea>
                        </div>
                      </div>
                      <div class="col-md-12">
                        <div class="text-start">
                          <a class="ud-btn btn-thm" href="page-contact.html">Save<i class="fal fa-arrow-right-long"></i></a>
                        </div>
                      </div>
                    </div>
                  </form>
                </div>
              </div>
              <div class="ps-widget bgc-white bdrs4 p30 mb30 overflow-hidden position-relative">
                <div class="bdrb1 pb15 mb25">
                  <h5 class="list-title">Skills</h5>
                </div>
                <div class="col-lg-7">
                  <div class="row">
                    <form class="form-style1">
                      <div class="row">
                        <div class="col-sm-6">
                          <div class="mb20">
                            <div class="form-style1">
                              <label class="heading-color ff-heading fw500 mb10">Skills 1</label>
                              <div class="bootselect-multiselect">
                                <select class="selectpicker">
                                  <option>Designer</option>
                                  <option>UI/UX</option>
                                </select>
                              </div>
                            </div>
                          </div>
                        </div>
                        <div class="col-sm-6">
                          <div class="mb20">
                            <div class="form-style1">
                              <label class="heading-color ff-heading fw500 mb10">Point</label>
                              <div class="bootselect-multiselect">
                                <select class="selectpicker">
                                  <option>90</option>
                                  <option>80</option>
                                </select>
                              </div>
                            </div>
                          </div>
                        </div>
                        <div class="col-sm-6">
                          <div class="mb20">
                            <div class="form-style1">
                              <label class="heading-color ff-heading fw500 mb10">Skills 2</label>
                              <div class="bootselect-multiselect">
                                <select class="selectpicker">
                                  <option>Developer</option>
                                  <option>Programmer</option>
                                </select>
                              </div>
                            </div>
                          </div>
                        </div>
                        <div class="col-sm-6">
                          <div class="mb20">
                            <div class="form-style1">
                            <label class="heading-color ff-heading fw500 mb10">Point</label>
                              <div class="bootselect-multiselect">
                                <select class="selectpicker">
                                  <option>60</option>
                                  <option>70</option>
                                </select>
                              </div>
                            </div>
                          </div>
                        </div>
                        <div class="col-sm-6">
                          <div class="mb20">
                            <div class="form-style1">
                              <label class="heading-color ff-heading fw500 mb10">Skills 3</label>
                              <div class="bootselect-multiselect">
                                <select class="selectpicker">
                                  <option>Video Editor</option>
                                  <option>Audio & Music</option>
                                </select>
                              </div>
                            </div>
                          </div>
                        </div>
                        <div class="col-sm-6">
                          <div class="mb20">
                            <div class="form-style1">
                              <label class="heading-color ff-heading fw500 mb10">Point</label>
                              <div class="bootselect-multiselect">
                                <select class="selectpicker">
                                  <option>75</option>
                                  <option>80</option>
                                </select>
                              </div>
                            </div>
                          </div>
                        </div>
                        <div class="col-md-12">
                          <div class="text-start">
                            <a class="ud-btn btn-thm" href="page-contact.html">Save<i class="fal fa-arrow-right-long"></i></a>
                          </div>
                        </div>
                      </div>
                    </form>
                  </div>
                </div>
              </div>
              <div class="ps-widget bgc-white bdrs4 p30 mb30 overflow-hidden position-relative">
                <div class="bdrb1 pb15 mb30 d-sm-flex justify-content-between">
                  <h5 class="list-title">Education</h5>
                  <a href="" class="add-more-btn text-thm"><i class="icon far fa-plus mr10"></i>Add Aducation</a>
                </div>
                <div class="position-relative">
                  <div class="educational-quality">
                    <div class="m-circle text-thm">M</div>
                    <div class="wrapper mb40 position-relative">
                      <div class="del-edit">
                        <div class="d-flex">
                          <a href="" class="icon me-2" data-bs-toggle="tooltip" data-bs-placement="top" title="" data-bs-original-title="Edit" aria-label="Edit"><span class="flaticon-pencil"></span></a>
                          <a href="" class="icon" data-bs-toggle="tooltip" data-bs-placement="top" title="" data-bs-original-title="Delete" aria-label="Delete"><span class="flaticon-delete"></span></a>
                        </div>
                      </div>
                      <span class="tag">2012 - 2014</span>
                      <h5 class="mt15">Bachlors in Fine Arts</h5>
                      <h6 class="text-thm">Modern College</h6>
                      <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin a ipsum tellus. Interdum et malesuada fames ac ante ipsum <br class="d-none d-lg-block"> primis in faucibus.</p>
                    </div>
                    <div class="m-circle before-none text-thm">M</div>
                    <div class="wrapper mb30 position-relative">
                      <div class="del-edit">
                        <div class="d-flex">
                          <a href="" class="icon me-2" data-bs-toggle="tooltip" data-bs-placement="top" title="" data-bs-original-title="Edit" aria-label="Edit"><span class="flaticon-pencil"></span></a>
                          <a href="" class="icon" data-bs-toggle="tooltip" data-bs-placement="top" title="" data-bs-original-title="Delete" aria-label="Delete"><span class="flaticon-delete"></span></a>
                        </div>
                      </div>
                      <span class="tag">2008 - 2012</span>
                      <h5 class="mt15">Computer Science</h5>
                      <h6 class="text-thm">Harvartd University</h6>
                      <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin a ipsum tellus. Interdum et malesuada fames ac ante ipsum <br class="d-none d-lg-block"> primis in faucibus.</p>
                    </div>
                  </div>
                  <div class="text-start">
                    <a class="ud-btn btn-thm" href="">Save<i class="fal fa-arrow-right-long"></i></a>
                  </div>
                </div>
              </div>
              <div class="ps-widget bgc-white bdrs4 p30 mb30 overflow-hidden position-relative">
                <div class="bdrb1 pb15 mb30 d-sm-flex justify-content-between">
                  <h5 class="list-title">Work & Experience</h5>
                  <a href="" class="add-more-btn text-thm"><i class="icon far fa-plus mr10"></i>Add Experience</a>
                </div>
                <div class="position-relative">
                  <div class="educational-quality">
                    <div class="m-circle text-thm">M</div>
                    <div class="wrapper mb40 position-relative">
                      <div class="del-edit">
                        <div class="d-flex">
                          <a href="" class="icon me-2" data-bs-toggle="tooltip" data-bs-placement="top" title="" data-bs-original-title="Edit" aria-label="Edit"><span class="flaticon-pencil"></span></a>
                          <a href="" class="icon" data-bs-toggle="tooltip" data-bs-placement="top" title="" data-bs-original-title="Delete" aria-label="Delete"><span class="flaticon-delete"></span></a>
                        </div>
                      </div>
                      <span class="tag">2012 - 2014</span>
                      <h5 class="mt15">UX Designer</h5>
                      <h6 class="text-thm">Dropbox</h6>
                      <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin a ipsum tellus. Interdum et malesuada fames ac ante ipsum <br class="d-none d-lg-block"> primis in faucibus.</p>
                    </div>
                    <div class="m-circle before-none text-thm">M</div>
                    <div class="wrapper mb30 position-relative">
                      <div class="del-edit">
                        <div class="d-flex">
                          <a href="" class="icon me-2" data-bs-toggle="tooltip" data-bs-placement="top" title="" data-bs-original-title="Edit" aria-label="Edit"><span class="flaticon-pencil"></span></a>
                          <a href="" class="icon" data-bs-toggle="tooltip" data-bs-placement="top" title="" data-bs-original-title="Delete" aria-label="Delete"><span class="flaticon-delete"></span></a>
                        </div>
                      </div>
                      <span class="tag">2008 - 2012</span>
                      <h5 class="mt15">Art Director</h5>
                      <h6 class="text-thm">amazon</h6>
                      <p class="mb-0">Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin a ipsum tellus. Interdum et malesuada fames ac ante ipsum <br class="d-none d-lg-block"> primis in faucibus.</p>
                    </div>
                  </div>
                  <div class="text-start">
                    <a class="ud-btn btn-thm" href="">Save<i class="fal fa-arrow-right-long"></i></a>
                  </div>
                </div>
              </div>
              <div class="ps-widget bgc-white bdrs4 p30 mb30 overflow-hidden position-relative">
                <div class="bdrb1 pb15 mb30 d-sm-flex justify-content-between">
                  <h5 class="list-title">Awards</h5>
                  <a href="" class="add-more-btn text-thm"><i class="icon far fa-plus mr10"></i>Add Awards</a>
                </div>
                <div class="position-relative">
                  <div class="educational-quality ps-0">
                    <div class="wrapper mb40 position-relative">
                      <div class="del-edit">
                        <div class="d-flex">
                          <a href="" class="icon me-2" data-bs-toggle="tooltip" data-bs-placement="top" title="" data-bs-original-title="Edit" aria-label="Edit"><span class="flaticon-pencil"></span></a>
                          <a href="" class="icon" data-bs-toggle="tooltip" data-bs-placement="top" title="" data-bs-original-title="Delete" aria-label="Delete"><span class="flaticon-delete"></span></a>
                        </div>
                      </div>
                      <span class="tag">2012 - 2014</span>
                      <h5 class="mt15">UI UX Design</h5>
                      <h6 class="text-thm">Udemy</h6>
                      <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin a ipsum tellus. Interdum et malesuada fames ac ante ipsum <br class="d-none d-lg-block"> primis in faucibus.</p>
                    </div>
                    <div class="wrapper mb40 position-relative">
                      <div class="del-edit">
                        <div class="d-flex">
                          <a href="" class="icon me-2" data-bs-toggle="tooltip" data-bs-placement="top" title="" data-bs-original-title="Edit" aria-label="Edit"><span class="flaticon-pencil"></span></a>
                          <a href="" class="icon" data-bs-toggle="tooltip" data-bs-placement="top" title="" data-bs-original-title="Delete" aria-label="Delete"><span class="flaticon-delete"></span></a>
                        </div>
                      </div>
                      <span class="tag">2008 - 2012</span>
                      <h5 class="mt15">App Design</h5>
                      <h6 class="text-thm">Google</h6>
                      <p class="mb-0">Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin a ipsum tellus. Interdum et malesuada fames ac ante ipsum <br class="d-none d-lg-block"> primis in faucibus.</p>
                    </div>
                  </div>
                  <div class="text-start">
                    <a class="ud-btn btn-thm" href="">Save<i class="fal fa-arrow-right-long"></i></a>
                  </div>
                </div>
              </div>
              <div class="ps-widget bgc-white bdrs4 p30 mb30 overflow-hidden position-relative">
                <div class="bdrb1 pb15 mb25">
                  <h5 class="list-title">Change password</h5>
                </div>
                <div class="col-lg-7">
                  <div class="row">
                    <form class="form-style1">
                      <div class="row">
                        <div class="col-sm-6">
                          <div class="mb20">
                            <label class="heading-color ff-heading fw500 mb10">Old Password</label>
                            <input type="text" class="form-control" placeholder="********">
                          </div>
                        </div>
                      </div>
                      <div class="row">
                        <div class="col-sm-12">
                          <div class="mb20">
                            <label class="heading-color ff-heading fw500 mb10">New Password</label>
                            <input type="text" class="form-control" placeholder="********">
                          </div>
                        </div>
                        <div class="col-sm-12">
                          <div class="mb20">
                            <label class="heading-color ff-heading fw500 mb10">Confirm New Password</label>
                            <input type="text" class="form-control" placeholder="********">
                          </div>
                        </div>
                        <div class="col-md-12">
                          <div class="text-start">
                            <a class="ud-btn btn-thm" href="page-contact.html">Change Password<i class="fal fa-arrow-right-long"></i></a>
                          </div>
                        </div>
                      </div>
                    </form>
                  </div>
                </div>
              </div>
              <div class="ps-widget bgc-white bdrs4 p30 mb30 overflow-hidden position-relative">
                <div class="col-lg-7">
                  <div class="row">
                    <div class="bdrb1 pb15 mb25">
                      <h5 class="list-title">Change password</h5>
                    </div>
                    <form class="form-style1">
                      <div class="row">
                        <div class="col-sm-12">
                          <h6>Close account</h6>
                          <p class="text">Warning: If you close your account, you will be unsubscribed from all your 5 courses, and will lose access forever.</p>
                        </div>
                        <div class="col-sm-6">
                          <div class="mb20">
                            <label class="heading-color ff-heading fw500 mb10">Enter Password</label>
                            <input type="text" class="form-control" placeholder="********">
                          </div>
                          <div class="text-start">
                            <a class="ud-btn btn-thm" href="page-contact.html">Change Password<i class="fal fa-arrow-right-long"></i></a>
                          </div>
                        </div>
                      </div>
                    </form>
                  </div>
                </div>
              </div>
            </div>
          </div>
</template>