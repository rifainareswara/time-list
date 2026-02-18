<template>
  <div>
    <!-- Header -->
    <div class="flex items-center justify-between mb-6">
      <div>
        <h1 class="text-3xl font-black text-white tracking-tight">All Tasks</h1>
        <p class="text-slate-500 text-sm mt-1">Overview of all tasks across all users</p>
      </div>
    </div>

    <!-- Summary Cards -->
    <div class="grid grid-cols-4 gap-4 mb-6">
      <div class="bg-slate-900 border border-slate-800 rounded-xl p-4">
        <div class="text-slate-500 text-xs uppercase tracking-widest mb-1">Total Tasks</div>
        <div class="text-3xl font-black text-white">{{ tasks.length }}</div>
      </div>
      <div class="bg-slate-900 border border-slate-800 rounded-xl p-4">
        <div class="text-slate-500 text-xs uppercase tracking-widest mb-1">Pending</div>
        <div class="text-3xl font-black text-yellow-400">{{ countByStatus('pending') }}</div>
      </div>
      <div class="bg-slate-900 border border-slate-800 rounded-xl p-4">
        <div class="text-slate-500 text-xs uppercase tracking-widest mb-1">In Progress</div>
        <div class="text-3xl font-black text-blue-400">{{ countByStatus('in_progress') }}</div>
      </div>
      <div class="bg-slate-900 border border-slate-800 rounded-xl p-4">
        <div class="text-slate-500 text-xs uppercase tracking-widest mb-1">Total Time</div>
        <div class="text-3xl font-black text-emerald-400">{{ totalHours }}h</div>
      </div>
    </div>

    <!-- Filters -->
    <div class="flex gap-3 mb-4">
      <select
        v-model="filterUser"
        class="bg-slate-900 border border-slate-700 rounded-lg px-3 py-2 text-sm text-slate-300 focus:outline-none focus:border-blue-500"
      >
        <option value="">All Users</option>
        <option v-for="u in uniqueUsers" :key="u" :value="u">{{ u }}</option>
      </select>

      <select
        v-model="filterStatus"
        class="bg-slate-900 border border-slate-700 rounded-lg px-3 py-2 text-sm text-slate-300 focus:outline-none focus:border-blue-500"
      >
        <option value="">All Statuses</option>
        <option value="pending">Pending</option>
        <option value="in_progress">In Progress</option>
        <option value="done">Done</option>
      </select>

      <select
        v-model="filterPriority"
        class="bg-slate-900 border border-slate-700 rounded-lg px-3 py-2 text-sm text-slate-300 focus:outline-none focus:border-blue-500"
      >
        <option value="">All Priorities</option>
        <option value="urgent">Urgent</option>
        <option value="high">High</option>
        <option value="normal">Normal</option>
        <option value="low">Low</option>
      </select>

      <input
        v-model="search"
        type="text"
        placeholder="Search tasks..."
        class="flex-1 bg-slate-900 border border-slate-700 rounded-lg px-3 py-2 text-sm text-slate-300 placeholder-slate-600 focus:outline-none focus:border-blue-500"
      />

      <button
        v-if="filterUser || filterStatus || filterPriority || search"
        @click="clearFilters"
        class="px-3 py-2 rounded-lg text-slate-400 hover:text-white hover:bg-slate-800 text-sm transition-colors"
      >
        Clear
      </button>
    </div>

    <!-- Table -->
    <div class="bg-slate-900 rounded-xl border border-slate-800 overflow-hidden">
      <div v-if="loading" class="p-8 text-center text-slate-500">Loading tasks...</div>
      <div v-else-if="filteredTasks.length === 0" class="p-8 text-center text-slate-500">No tasks found.</div>
      <table v-else class="w-full text-sm">
        <thead>
          <tr class="border-b border-slate-800 text-slate-500 text-xs uppercase tracking-wide">
            <th class="text-left px-5 py-3">User</th>
            <th class="text-left px-5 py-3">Task</th>
            <th class="text-left px-5 py-3">Project</th>
            <th class="text-left px-5 py-3">Status</th>
            <th class="text-left px-5 py-3">Priority</th>
            <th class="text-left px-5 py-3">Due Date</th>
            <th class="text-right px-5 py-3">Time</th>
            <th class="text-right px-5 py-3">Subtasks</th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="task in filteredTasks"
            :key="task.id"
            class="border-b border-slate-800/50 hover:bg-slate-800/30 transition-colors"
          >
            <!-- User -->
            <td class="px-5 py-3">
              <div class="text-slate-200 text-xs font-semibold">{{ task.full_name || task.username }}</div>
              <div class="text-slate-500 text-xs">@{{ task.username }}</div>
            </td>

            <!-- Task title + description -->
            <td class="px-5 py-3 max-w-xs">
              <div class="text-slate-200 font-medium truncate">{{ task.title }}</div>
              <div v-if="task.description" class="text-slate-500 text-xs truncate mt-0.5">{{ task.description }}</div>
            </td>

            <!-- Project -->
            <td class="px-5 py-3">
              <span v-if="task.project_name" class="inline-flex items-center gap-1.5 text-xs">
                <span class="w-2 h-2 rounded-full flex-shrink-0" :style="{ background: task.project_color || '#64748b' }"></span>
                <span class="text-slate-400">{{ task.project_name }}</span>
              </span>
              <span v-else class="text-slate-600 text-xs">—</span>
            </td>

            <!-- Status -->
            <td class="px-5 py-3">
              <span class="px-2 py-0.5 rounded text-xs font-semibold uppercase" :class="statusClass(task.status)">
                {{ task.status.replace('_', ' ') }}
              </span>
            </td>

            <!-- Priority -->
            <td class="px-5 py-3">
              <span class="px-2 py-0.5 rounded text-xs font-semibold uppercase" :class="priorityClass(task.priority)">
                {{ task.priority }}
              </span>
            </td>

            <!-- Due date -->
            <td class="px-5 py-3 text-slate-400 text-xs">
              <span v-if="task.due_date" :class="isOverdue(task) ? 'text-red-400 font-semibold' : ''">
                {{ formatDate(task.due_date) }}
              </span>
              <span v-else class="text-slate-600">—</span>
            </td>

            <!-- Time -->
            <td class="px-5 py-3 text-right text-slate-400 text-xs">
              {{ formatMinutes(task.total_minutes) }}
            </td>

            <!-- Subtasks -->
            <td class="px-5 py-3 text-right text-xs">
              <span v-if="task.subtask_count > 0" :class="task.subtask_done === task.subtask_count ? 'text-emerald-400' : 'text-slate-400'">
                {{ task.subtask_done }}/{{ task.subtask_count }}
              </span>
              <span v-else class="text-slate-600">—</span>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Footer count -->
    <div class="mt-3 text-xs text-slate-600 text-right">
      Showing {{ filteredTasks.length }} of {{ tasks.length }} tasks
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useAuthStore } from '~/stores/auth'

const auth = useAuthStore()
const tasks = ref([])
const loading = ref(true)
const filterUser = ref('')
const filterStatus = ref('')
const filterPriority = ref('')
const search = ref('')

onMounted(async () => {
  if (!auth.isAdmin) {
    navigateTo('/')
    return
  }
  await fetchAllTasks()
})

async function fetchAllTasks() {
  loading.value = true
  const { data, error } = await useFetch('/api/admin/tasks', {
    headers: { Authorization: `Bearer ${auth.token}` }
  })
  if (data.value) {
    tasks.value = data.value
  }
  if (error.value) {
    console.error('Failed to fetch admin tasks', error.value)
  }
  loading.value = false
}

// ── Computed ──
const uniqueUsers = computed(() => [...new Set(tasks.value.map(t => t.username))].sort())

const filteredTasks = computed(() => {
  return tasks.value.filter(t => {
    if (filterUser.value && t.username !== filterUser.value) return false
    if (filterStatus.value && t.status !== filterStatus.value) return false
    if (filterPriority.value && t.priority !== filterPriority.value) return false
    if (search.value) {
      const q = search.value.toLowerCase()
      if (!t.title.toLowerCase().includes(q) && !(t.description || '').toLowerCase().includes(q)) return false
    }
    return true
  })
})

const totalHours = computed(() => {
  const mins = tasks.value.reduce((acc, t) => acc + (t.total_minutes || 0), 0)
  return (mins / 60).toFixed(1)
})

function countByStatus(status) {
  return tasks.value.filter(t => t.status === status).length
}

function clearFilters() {
  filterUser.value = ''
  filterStatus.value = ''
  filterPriority.value = ''
  search.value = ''
}

// ── Helpers ──
function statusClass(status) {
  return {
    pending: 'bg-yellow-900/30 text-yellow-400',
    in_progress: 'bg-blue-900/30 text-blue-400',
    done: 'bg-emerald-900/30 text-emerald-400',
  }[status] || 'bg-slate-800 text-slate-400'
}

function priorityClass(priority) {
  return {
    urgent: 'bg-red-900/30 text-red-400',
    high: 'bg-orange-900/30 text-orange-400',
    normal: 'bg-slate-800 text-slate-400',
    low: 'bg-slate-800/50 text-slate-600',
  }[priority] || 'bg-slate-800 text-slate-400'
}

function isOverdue(task) {
  if (!task.due_date || task.status === 'done') return false
  return new Date(task.due_date) < new Date()
}

function formatDate(dateStr) {
  if (!dateStr) return '—'
  return new Date(dateStr).toLocaleDateString('en-GB', { day: '2-digit', month: 'short', year: 'numeric' })
}

function formatMinutes(mins) {
  if (!mins || mins === 0) return '—'
  const h = Math.floor(mins / 60)
  const m = mins % 60
  return h > 0 ? `${h}h ${m}m` : `${m}m`
}
</script>
