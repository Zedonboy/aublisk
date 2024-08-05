<template>
    <div class="dropdown">
        <button onclick="myFunction()" class="dropbtn"><i class="fa fa-bars pr10"></i> Dashboard Navigation</button>
        <ul id="myDropdown" class="dropdown-content">
            <li>
                <p class="fz15 fw400 ff-heading mt30 pl30">Start</p>
            </li>
            <RouterLink to="/"  exact-active-class="active"><i class="flaticon-home mr10"></i>Dashboard</RouterLink>
            <RouterLink exact-active-class="active" to="/manage/project"><i class="flaticon-document mr10"></i>Manage Project</RouterLink>
            
            <li>
                <p class="fz15 fw400 ff-heading mt30 pl30">Freelancer and DAO</p>
            </li>
            <RouterLink to="/wallet"><i class="flaticon-presentation mr10"></i>Wallet & Payout</RouterLink>
            <RouterLink to="/dao"><i class="flaticon-briefcase mr10"></i>Manage DAO</RouterLink>
            <RouterLink to="/profile"><i class="flaticon-content mr10"></i>My Profile
            </RouterLink>
            <li><a href="page-login.html"><i class="flaticon-logout mr10"></i>Logout</a></li>
        </ul>
    </div>
</template>