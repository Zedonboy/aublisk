<template>
  
  <RouterView/>

</template>

<script setup>
import { AuthClient } from '@dfinity/auth-client';
import { onMounted } from 'vue';

onMounted(() => {
  let task = async () => {
    let client = await AuthClient.create()
    let is_authenticated = await client.isAuthenticated()
    if(is_authenticated) {
      // go to login
    }
  }
})
</script>