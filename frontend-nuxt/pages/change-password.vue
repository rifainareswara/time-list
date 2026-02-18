<template>
  <div class="min-h-screen flex items-center justify-center bg-slate-950 p-6">
    <div class="w-full max-w-md bg-slate-900 rounded-2xl border border-slate-800 p-8 shadow-2xl">
      <div class="text-center mb-8">
        <h1 class="text-3xl font-black text-white tracking-tight mb-2">Change Password</h1>
        <p class="text-amber-400 font-bold">Security Alert</p>
        <p class="text-slate-400 text-sm mt-2">Your password was reset by an administrator. You must set a new password to continue.</p>
      </div>

      <form @submit.prevent="handleChangePassword" class="space-y-6">
        <div>
          <label class="block text-sm font-bold text-slate-400 mb-2 uppercase tracking-wide">Current (Temp) Password</label>
          <input 
            v-model="oldPassword" 
            type="password" 
            class="w-full bg-slate-950 border border-slate-700 text-white rounded-lg px-4 py-3 focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition-all"
            placeholder="••••••••"
            required
          >
        </div>
        
        <div>
          <label class="block text-sm font-bold text-slate-400 mb-2 uppercase tracking-wide">New Password</label>
          <input 
            v-model="newPassword" 
            type="password" 
            class="w-full bg-slate-950 border border-slate-700 text-white rounded-lg px-4 py-3 focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition-all"
            placeholder="••••••••"
            required
          >
        </div>

        <div>
           <label class="block text-sm font-bold text-slate-400 mb-2 uppercase tracking-wide">Confirm New Password</label>
           <input 
             v-model="confirmPassword" 
             type="password" 
             class="w-full bg-slate-950 border border-slate-700 text-white rounded-lg px-4 py-3 focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition-all"
             placeholder="••••••••"
             required
           >
         </div>

        <div v-if="error" class="p-3 bg-red-900/30 border border-red-800 rounded-lg text-red-400 text-sm text-center">
          {{ error }}
        </div>

        <button 
          type="submit" 
          :disabled="loading"
          class="w-full bg-blue-600 hover:bg-blue-500 text-white font-bold py-3.5 rounded-xl transition-all shadow-lg hover:shadow-blue-500/25 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {{ loading ? 'Updating Password...' : 'Update Password' }}
        </button>
      </form>
      
      <div class="mt-6 text-center">
        <button @click="auth.logout()" class="text-slate-500 hover:text-slate-400 text-sm font-bold hover:underline">
           Sign Out
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useAuthStore } from '~/stores/auth'

definePageMeta({
  layout: false
})

const auth = useAuthStore()
const oldPassword = ref('')
const newPassword = ref('')
const confirmPassword = ref('')
const loading = ref(false)
const error = ref('')

async function handleChangePassword() {
  if (newPassword.value !== confirmPassword.value) {
    error.value = 'New passwords do not match'
    return
  }
  
  if (newPassword.value.length < 6) {
    error.value = 'Password must be at least 6 characters'
    return
  }

  loading.value = true
  error.value = ''
  
  const success = await auth.changePassword(oldPassword.value, newPassword.value)
  if (success) {
    navigateTo('/')
  } else {
    error.value = 'Failed to update password. Check your old password.'
  }
  
  loading.value = false
}
</script>
