<template>
  <div>
    <h1 class="text-3xl font-black text-white tracking-tight mb-6">User Management</h1>

    <div class="bg-slate-900 rounded-xl border border-slate-800 overflow-hidden">
      <table class="w-full text-sm">
        <thead>
          <tr class="border-b border-slate-800 text-slate-500 text-xs uppercase tracking-wide">
            <th class="text-left px-6 py-4">Username</th>
            <th class="text-left px-6 py-4">Role</th>
            <th class="text-left px-6 py-4">Created At</th>
            <th class="text-right px-6 py-4">Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="user in users" :key="user.id" class="border-b border-slate-800/50 hover:bg-slate-800/30">
            <td class="px-6 py-4 text-slate-200 font-medium">{{ user.username }}</td>
            <td class="px-6 py-4">
              <button 
                @click="toggleRole(user)"
                class="px-2 py-0.5 rounded text-xs font-bold uppercase transition-colors cursor-pointer"
                :class="user.role === 'admin' ? 'bg-purple-900/30 text-purple-400 hover:bg-purple-900/50' : 'bg-slate-800 text-slate-400 hover:bg-slate-700'"
                :disabled="user.id === auth.user.id"
                title="Click to toggle role"
              >
                {{ user.role }}
              </button>
            </td>
            <td class="px-6 py-4 text-slate-400">{{ new Date(user.created_at).toLocaleDateString() }}</td>
            <td class="px-6 py-4 text-right space-x-3">
              <button 
                @click="promptResetPassword(user)"
                class="text-blue-400 hover:text-blue-300 font-bold hover:underline"
              >
                Reset Password
              </button>
              <button 
                v-if="user.id !== auth.user.id"
                @click="deleteUser(user.id)"
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
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useAuthStore } from '~/stores/auth'

const auth = useAuthStore()
const users = ref([])

onMounted(async () => {
  if (!auth.isAdmin) {
    navigateTo('/')
    return
  }
  await fetchUsers()
})

async function fetchUsers() {
  const { data, error } = await useFetch('/api/users', {
    headers: { Authorization: `Bearer ${auth.token}` }
  })
  
  if (data.value) {
    users.value = data.value
  }
}

async function deleteUser(id) {
  if (!confirm('Are you sure you want to delete this user? This cannot be undone.')) return

  const { error } = await useFetch(`/api/users/${id}`, {
    method: 'DELETE',
    headers: { Authorization: `Bearer ${auth.token}` }
  })

  if (!error.value) {
    await fetchUsers()
  }
}

async function toggleRole(user) {
  if (user.id === auth.user.id) return // Validated in template too

  const newRole = user.role === 'admin' ? 'user' : 'admin'
  const confirmMsg = `Promote ${user.username} to Admin?` 
  const confirmMsg2 = `Demote ${user.username} to User?`
  
  if (!confirm(newRole === 'admin' ? confirmMsg : confirmMsg2)) return

  const { error } = await useFetch(`/api/users/${user.id}/role`, {
    method: 'PUT',
    headers: { Authorization: `Bearer ${auth.token}` },
    body: { role: newRole }
  })

  if (!error.value) {
    await fetchUsers()
  } else {
    alert('Failed to update role')
  }
}

async function promptResetPassword(user) {
  const newPassword = prompt(`Enter new password for ${user.username}:`)
  if (!newPassword) return

  const { error } = await useFetch(`/api/users/${user.id}/password`, {
    method: 'PUT',
    headers: { Authorization: `Bearer ${auth.token}` },
    body: { new_password: newPassword }
  })

  if (!error.value) {
    alert('Password reset successfully')
  } else {
    alert('Failed to reset password')
  }
}
</script>
