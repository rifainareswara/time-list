<template>
  <div>
    <h1 class="text-3xl font-black text-white tracking-tight mb-6">User Management</h1>

    <div class="bg-slate-900 rounded-xl border border-slate-800 overflow-hidden">
      <table class="w-full text-sm">
        <thead>
          <tr class="border-b border-slate-800 text-slate-500 text-xs uppercase tracking-wide">
            <th class="text-left px-6 py-4">Name</th>
            <th class="text-left px-6 py-4">Username</th>
            <th class="text-left px-6 py-4">Role</th>
            <th class="text-left px-6 py-4">Created At</th>
            <th class="text-right px-6 py-4">Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="user in users" :key="user.id" class="border-b border-slate-800/50 hover:bg-slate-800/30">
            <td class="px-6 py-4">
              <div class="text-slate-200 font-semibold">{{ user.full_name || '—' }}</div>
              <div class="text-slate-500 text-xs">@{{ user.username }}</div>
            </td>
            <td class="px-6 py-4 text-slate-400 text-xs font-mono">{{ user.username }}</td>
            <td class="px-6 py-4">
              <button 
                @click="openRoleModal(user)"
                class="px-2 py-0.5 rounded text-xs font-bold uppercase transition-colors cursor-pointer"
                :class="user.role === 'admin' ? 'bg-purple-900/30 text-purple-400 hover:bg-purple-900/50' : 'bg-slate-800 text-slate-400 hover:bg-slate-700'"
                :disabled="user.id === auth.user?.id"
                title="Click to toggle role"
              >
                {{ user.role }}
              </button>
            </td>
            <td class="px-6 py-4 text-slate-400">{{ new Date(user.created_at).toLocaleDateString() }}</td>
            <td class="px-6 py-4 text-right space-x-3">
              <button 
                @click="openResetModal(user)"
                class="text-blue-400 hover:text-blue-300 font-bold hover:underline"
              >
                Reset Password
              </button>
              <button 
                v-if="user.id !== auth.user?.id"
                @click="openDeleteModal(user)"
                class="text-red-400 hover:text-red-300 font-bold hover:underline"
              >
                Delete
              </button>
              <span v-else class="text-slate-600 italic">Current User</span>
            </td>
          </tr>
        </tbody>
      </table>

      <div v-if="users.length === 0" class="p-8 text-center text-slate-500">
        No users found.
      </div>
    </div>

    <!-- Reset Password Modal -->
    <div v-if="resetModal.open" class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm">
      <div class="bg-slate-900 border border-slate-700 rounded-2xl p-6 w-full max-w-md shadow-2xl">
        <h2 class="text-lg font-bold text-white mb-1">Reset Password</h2>
        <p class="text-slate-400 text-sm mb-4">Set a new password for <span class="text-white font-semibold">{{ resetModal.user?.username }}</span>.</p>
        <input
          v-model="resetModal.newPassword"
          type="password"
          placeholder="New password"
          class="w-full bg-slate-800 border border-slate-700 rounded-lg px-4 py-2.5 text-white placeholder-slate-500 focus:outline-none focus:border-blue-500 mb-4"
          @keyup.enter="confirmReset"
        />
        <p v-if="resetModal.error" class="text-red-400 text-sm mb-3">{{ resetModal.error }}</p>
        <div class="flex gap-3 justify-end">
          <button @click="resetModal.open = false" class="px-4 py-2 rounded-lg text-slate-400 hover:text-white hover:bg-slate-800 transition-colors">Cancel</button>
          <button @click="confirmReset" :disabled="resetModal.loading" class="px-4 py-2 rounded-lg bg-blue-600 hover:bg-blue-500 text-white font-semibold transition-colors disabled:opacity-50">
            {{ resetModal.loading ? 'Saving...' : 'Reset Password' }}
          </button>
        </div>
      </div>
    </div>

    <!-- Role Change Modal -->
    <div v-if="roleModal.open" class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm">
      <div class="bg-slate-900 border border-slate-700 rounded-2xl p-6 w-full max-w-md shadow-2xl">
        <h2 class="text-lg font-bold text-white mb-1">Change Role</h2>
        <p class="text-slate-400 text-sm mb-4">
          Change <span class="text-white font-semibold">{{ roleModal.user?.username }}</span>'s role from
          <span class="font-semibold" :class="roleModal.user?.role === 'admin' ? 'text-purple-400' : 'text-slate-300'">{{ roleModal.user?.role }}</span>
          to
          <span class="font-semibold" :class="roleModal.newRole === 'admin' ? 'text-purple-400' : 'text-slate-300'">{{ roleModal.newRole }}</span>?
        </p>
        <p v-if="roleModal.error" class="text-red-400 text-sm mb-3">{{ roleModal.error }}</p>
        <div class="flex gap-3 justify-end">
          <button @click="roleModal.open = false" class="px-4 py-2 rounded-lg text-slate-400 hover:text-white hover:bg-slate-800 transition-colors">Cancel</button>
          <button @click="confirmRoleChange" :disabled="roleModal.loading" class="px-4 py-2 rounded-lg bg-purple-600 hover:bg-purple-500 text-white font-semibold transition-colors disabled:opacity-50">
            {{ roleModal.loading ? 'Saving...' : 'Confirm' }}
          </button>
        </div>
      </div>
    </div>

    <!-- Delete User Modal -->
    <div v-if="deleteModal.open" class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm">
      <div class="bg-slate-900 border border-slate-700 rounded-2xl p-6 w-full max-w-md shadow-2xl">
        <h2 class="text-lg font-bold text-white mb-1">Delete User</h2>
        <p class="text-slate-400 text-sm mb-4">Are you sure you want to delete <span class="text-white font-semibold">{{ deleteModal.user?.username }}</span>? This cannot be undone.</p>
        <p v-if="deleteModal.error" class="text-red-400 text-sm mb-3">{{ deleteModal.error }}</p>
        <div class="flex gap-3 justify-end">
          <button @click="deleteModal.open = false" class="px-4 py-2 rounded-lg text-slate-400 hover:text-white hover:bg-slate-800 transition-colors">Cancel</button>
          <button @click="confirmDelete" :disabled="deleteModal.loading" class="px-4 py-2 rounded-lg bg-red-600 hover:bg-red-500 text-white font-semibold transition-colors disabled:opacity-50">
            {{ deleteModal.loading ? 'Deleting...' : 'Delete User' }}
          </button>
        </div>
      </div>
    </div>

    <!-- Success Toast -->
    <div v-if="toast.show" class="fixed bottom-6 right-6 z-50 bg-green-600 text-white px-5 py-3 rounded-xl shadow-xl font-semibold transition-all">
      {{ toast.message }}
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted } from 'vue'
import { useAuthStore } from '~/stores/auth'

const auth = useAuthStore()
const users = ref([])

const resetModal = reactive({ open: false, user: null, newPassword: '', loading: false, error: '' })
const roleModal = reactive({ open: false, user: null, newRole: '', loading: false, error: '' })
const deleteModal = reactive({ open: false, user: null, loading: false, error: '' })
const toast = reactive({ show: false, message: '' })

function showToast(message) {
  toast.message = message
  toast.show = true
  setTimeout(() => { toast.show = false }, 3000)
}

onMounted(async () => {
  if (!auth.isAdmin) {
    navigateTo('/')
    return
  }
  await fetchUsers()
})

async function fetchUsers() {
  const { data } = await useFetch('/api/users', {
    headers: { Authorization: `Bearer ${auth.token}` }
  })
  if (data.value) {
    users.value = data.value
  }
}

// ── Reset Password ──
function openResetModal(user) {
  resetModal.user = user
  resetModal.newPassword = ''
  resetModal.error = ''
  resetModal.loading = false
  resetModal.open = true
}

async function confirmReset() {
  if (!resetModal.newPassword.trim()) {
    resetModal.error = 'Password cannot be empty.'
    return
  }
  resetModal.loading = true
  resetModal.error = ''

  const { error } = await useFetch(`/api/users/${resetModal.user.id}/password`, {
    method: 'PUT',
    headers: { Authorization: `Bearer ${auth.token}`, 'Content-Type': 'application/json' },
    body: JSON.stringify({ new_password: resetModal.newPassword })
  })

  resetModal.loading = false
  if (!error.value) {
    resetModal.open = false
    showToast(`Password for ${resetModal.user.username} reset successfully.`)
  } else {
    resetModal.error = 'Failed to reset password. Please try again.'
  }
}

// ── Role Change ──
function openRoleModal(user) {
  if (user.id === auth.user?.id) return
  roleModal.user = user
  roleModal.newRole = user.role === 'admin' ? 'user' : 'admin'
  roleModal.error = ''
  roleModal.loading = false
  roleModal.open = true
}

async function confirmRoleChange() {
  roleModal.loading = true
  roleModal.error = ''

  const { error } = await useFetch(`/api/users/${roleModal.user.id}/role`, {
    method: 'PUT',
    headers: { Authorization: `Bearer ${auth.token}`, 'Content-Type': 'application/json' },
    body: JSON.stringify({ role: roleModal.newRole })
  })

  roleModal.loading = false
  if (!error.value) {
    roleModal.open = false
    showToast(`${roleModal.user.username} is now ${roleModal.newRole}.`)
    await fetchUsers()
  } else {
    roleModal.error = 'Failed to update role. Please try again.'
  }
}

// ── Delete User ──
function openDeleteModal(user) {
  deleteModal.user = user
  deleteModal.error = ''
  deleteModal.loading = false
  deleteModal.open = true
}

async function confirmDelete() {
  deleteModal.loading = true
  deleteModal.error = ''

  const { error } = await useFetch(`/api/users/${deleteModal.user.id}`, {
    method: 'DELETE',
    headers: { Authorization: `Bearer ${auth.token}` }
  })

  deleteModal.loading = false
  if (!error.value) {
    deleteModal.open = false
    showToast(`User ${deleteModal.user.username} deleted.`)
    await fetchUsers()
  } else {
    deleteModal.error = 'Failed to delete user. Please try again.'
  }
}
</script>
