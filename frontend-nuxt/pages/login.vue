<template>
  <div class="min-h-screen flex items-center justify-center bg-slate-950 p-6">
    <div class="w-full max-w-md bg-slate-900 rounded-2xl border border-slate-800 p-8 shadow-2xl">
      <div class="text-center mb-8">
        <h1 class="text-3xl font-black text-white tracking-tight mb-2">Welcome Back</h1>
        <p class="text-slate-400">Sign in to continue to Sic Mundus</p>
      </div>

      <form @submit.prevent="handleLogin" class="space-y-6">
        <div>
          <label class="block text-sm font-bold text-slate-400 mb-2 uppercase tracking-wide">Username</label>
          <input 
            v-model="username" 
            type="text" 
            class="w-full bg-slate-950 border border-slate-700 text-white rounded-lg px-4 py-3 focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition-all"
            placeholder="Enter your username"
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

        <div v-if="error" class="p-3 bg-red-900/30 border border-red-800 rounded-lg text-red-400 text-sm text-center">
          {{ error }}
        </div>

        <button 
          type="submit" 
          :disabled="loading"
          class="w-full bg-blue-600 hover:bg-blue-500 text-white font-bold py-3.5 rounded-xl transition-all shadow-lg hover:shadow-blue-500/25 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {{ loading ? 'Signing in...' : 'Sign In' }}
        </button>
      </form>

      <div class="mt-6 text-center text-sm text-slate-500">
        Don't have an account? 
        <NuxtLink to="/register" class="text-blue-400 hover:text-blue-300 font-bold hover:underline">Create Account</NuxtLink>
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
const username = ref('')
const password = ref('')
const loading = ref(false)
const error = ref('')

async function handleLogin() {
  loading.value = true
  error.value = ''
  
  const success = await auth.login(username.value, password.value)
  if (success) {
    navigateTo('/')
  } else {
    error.value = 'Invalid username or password'
  }
  
  loading.value = false
}
</script>
