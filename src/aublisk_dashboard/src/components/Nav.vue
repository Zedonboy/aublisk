<template>
    <div class="dashboard_sidebar_list">
          <p class="fz15 fw400 ff-heading pl30">Start</p>
          <div class="sidebar_list_item">
            <RouterLink active-class="-is-active" exact-active-class="-is-active" to="/" class="items-center"><i class="flaticon-home mr15"></i>Dashboard</RouterLink>
          </div>
          <div class="sidebar_list_item">
            <RouterLink to="/manage/project" exact-active-class="-is-active" class="items-center"><i class="flaticon-document mr15"></i>Manage Project</RouterLink>
          </div>
          
          <p class="fz15 fw400 ff-heading pl30 mt30">Freelancer and Dao</p>
          <div class="sidebar_list_item ">
            <RouterLink exact-active-class="-is-active" to="/wallet" class="items-center"><i class="flaticon-presentation mr15"></i>Wallet & Payouts</RouterLink>
          </div>
          <div class="sidebar_list_item ">
            <RouterLink exact-active-class="-is-active" to="/dao" class="items-center"><i class="flaticon-briefcase mr15"></i>Manage DAO</RouterLink>
          </div>
          <div class="sidebar_list_item ">
            <RouterLink exact-active-class="-is-active" to="/profile" class="items-center"><i class="flaticon-content mr15"></i>My Profile</RouterLink>
          </div>
          <!-- <p class="fz15 fw400 ff-heading pl30 mt30">Account</p>
          <div class="sidebar_list_item ">
            <a href="page-dashboard-profile.html" class="items-center"><i class="flaticon-photo mr15"></i>My Profile</a>
          </div>
          <div class="sidebar_list_item ">
            <a href="page-login.html" class="items-center"><i class="flaticon-logout mr15"></i>Logout</a>
          </div> -->
        </div>
</template>