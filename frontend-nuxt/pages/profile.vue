<template>
  <div class="max-w-lg mx-auto">
    <div class="mb-6">
      <h1 class="text-3xl font-black text-white tracking-tight">My Profile</h1>
      <p class="text-slate-500 text-sm mt-1">Update your display name</p>
    </div>

    <div class="bg-slate-900 border border-slate-800 rounded-xl p-6">
      <!-- Current info -->
      <div class="mb-5 pb-5 border-b border-slate-800">
        <div class="text-slate-500 text-xs uppercase tracking-widest mb-1">Username</div>
        <div class="text-white font-mono text-lg">{{ auth.user?.username }}</div>
      </div>

      <!-- Full name edit -->
      <div class="mb-5">
        <label class="block text-slate-400 text-sm font-medium mb-2">Full Name</label>
        <input
          v-model="fullName"
          type="text"
          placeholder="e.g. John Doe"
          class="w-full bg-slate-800 border border-slate-700 rounded-lg px-4 py-2.5 text-white placeholder-slate-500 focus:outline-none focus:border-blue-500 transition-colors"
          @keyup.enter="save"
        />
      </div>

      <p v-if="error" class="text-red-400 text-sm mb-3">{{ error }}</p>
      <p v-if="success" class="text-emerald-400 text-sm mb-3">Profile updated!</p>

      <button
        @click="save"
        :disabled="loading"
        class="w-full py-2.5 rounded-lg bg-blue-600 hover:bg-blue-500 text-white font-semibold transition-colors disabled:opacity-50"
      >
        {{ loading ? 'Saving...' : 'Save Changes' }}
      </button>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useAuthStore } from '~/stores/auth'

const auth = useAuthStore()
const fullName = ref('')
const loading = ref(false)
const error = ref('')
const success = ref(false)

onMounted(() => {
  fullName.value = auth.user?.full_name || ''
})

async function save() {
  if (!fullName.value.trim()) {
    error.value = 'Name cannot be empty.'
    return
  }
  loading.value = true
  error.value = ''
  success.value = false

  const { error: fetchError } = await useFetch('/api/me/profile', {
    method: 'PUT',
    headers: { Authorization: `Bearer ${auth.token}`, 'Content-Type': 'application/json' },
    body: JSON.stringify({ full_name: fullName.value.trim() })
  })

  loading.value = false
  if (fetchError.value) {
    error.value = 'Failed to save. Please try again.'
  } else {
    // Update local user state
    if (auth.user) auth.user.full_name = fullName.value.trim()
    success.value = true
    setTimeout(() => { success.value = false }, 3000)
  }
}
</script>
