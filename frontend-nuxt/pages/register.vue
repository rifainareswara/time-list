<template>
  <div class="min-h-screen flex items-center justify-center bg-slate-950 p-6">
    <div class="w-full max-w-md bg-slate-900 rounded-2xl border border-slate-800 p-8 shadow-2xl">
      <div class="text-center mb-8">
        <h1 class="text-3xl font-black text-white tracking-tight mb-2">Create Account</h1>
        <p class="text-slate-400">Join Sic Mundus and manage your time</p>
      </div>

      <form @submit.prevent="handleRegister" class="space-y-6">
        <div>
          <label class="block text-sm font-bold text-slate-400 mb-2 uppercase tracking-wide">Full Name</label>
          <input 
            v-model="fullName" 
            type="text" 
            class="w-full bg-slate-950 border border-slate-700 text-white rounded-lg px-4 py-3 focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition-all"
            placeholder="Your full name"
            required
          >
        </div>

        <div>
          <label class="block text-sm font-bold text-slate-400 mb-2 uppercase tracking-wide">Username</label>
          <input 
            v-model="username" 
            type="text" 
            class="w-full bg-slate-950 border border-slate-700 text-white rounded-lg px-4 py-3 focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition-all"
            placeholder="Choose a username"
            required
          >
        </div>
        
        <div>
          <label class="block text-sm font-bold text-slate-400 mb-2 uppercase tracking-wide">Password</label>
          <input 
            v-model="password" 
            type="password" 
            class="w-full bg-slate-950 border border-slate-700 text-white rounded-lg px-4 py-3 focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition-all"
            placeholder="••••••••"
            required
          >
        </div>

        <div>
           <label class="block text-sm font-bold text-slate-400 mb-2 uppercase tracking-wide">Confirm Password</label>
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
          class="w-full bg-emerald-600 hover:bg-emerald-500 text-white font-bold py-3.5 rounded-xl transition-all shadow-lg hover:shadow-emerald-500/25 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {{ loading ? 'Creating Account...' : 'Sign Up' }}
        </button>
      </form>

      <div class="mt-6 text-center text-sm text-slate-500">
        Already have an account? 
        <NuxtLink to="/login" class="text-blue-400 hover:text-blue-300 font-bold hover:underline">Sign In</NuxtLink>
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
const fullName = ref('')
const username = ref('')
const password = ref('')
const confirmPassword = ref('')
const loading = ref(false)
const error = ref('')

async function handleRegister() {
  if (password.value !== confirmPassword.value) {
    error.value = 'Passwords do not match'
    return
  }

  loading.value = true
  error.value = ''
  
  const success = await auth.register(username.value, password.value, fullName.value)
  if (success) {
    navigateTo('/')
  } else {
    error.value = 'Registration failed. Username may be taken.'
  }
  
  loading.value = false
}
</script>
