<template>
  <div>
    <!-- Header -->
    <div class="flex items-center justify-between mb-6">
      <div>
        <h1 class="text-3xl font-black text-white tracking-tight">Time Report</h1>
        <p class="text-slate-500 text-sm mt-1">Total hours logged per user per project</p>
      </div>
    </div>

    <!-- Summary Cards -->
    <div class="grid grid-cols-3 gap-4 mb-6">
      <div class="bg-slate-900 border border-slate-800 rounded-xl p-4">
        <div class="text-slate-500 text-xs uppercase tracking-widest mb-1">Total Users</div>
        <div class="text-3xl font-black text-white">{{ uniqueUsers.length }}</div>
      </div>
      <div class="bg-slate-900 border border-slate-800 rounded-xl p-4">
        <div class="text-slate-500 text-xs uppercase tracking-widest mb-1">Total Hours</div>
        <div class="text-3xl font-black text-emerald-400">{{ totalHours }}h</div>
      </div>
      <div class="bg-slate-900 border border-slate-800 rounded-xl p-4">
        <div class="text-slate-500 text-xs uppercase tracking-widest mb-1">Most Active</div>
        <div class="text-xl font-black text-blue-400 truncate">{{ mostActiveUser }}</div>
      </div>
    </div>

    <!-- Filter -->
    <div class="flex gap-3 mb-4">
      <select
        v-model="filterUser"
        class="bg-slate-900 border border-slate-700 rounded-lg px-3 py-2 text-sm text-slate-300 focus:outline-none focus:border-blue-500"
      >
        <option value="">All Users</option>
        <option v-for="u in uniqueUsers" :key="u.id" :value="u.id">{{ u.display }}</option>
      </select>
      <button
        v-if="filterUser"
        @click="filterUser = ''"
        class="px-3 py-2 rounded-lg text-slate-400 hover:text-white hover:bg-slate-800 text-sm transition-colors"
      >Clear</button>
    </div>

    <!-- Loading / Empty -->
    <div v-if="loading" class="p-8 text-center text-slate-500">Loading report...</div>
    <div v-else-if="groupedRows.length === 0" class="p-8 text-center text-slate-500">No time data found.</div>

    <!-- User Sections -->
    <div v-else class="space-y-6">
      <div
        v-for="group in groupedRows"
        :key="group.user_id"
        class="bg-slate-900 border border-slate-800 rounded-xl overflow-hidden"
      >
        <!-- User Header -->
        <div class="flex items-center justify-between px-5 py-3 border-b border-slate-800 bg-slate-800/40">
          <div>
            <span class="text-white font-bold">{{ group.display_name }}</span>
            <span class="text-slate-500 text-xs ml-2">@{{ group.username }}</span>
          </div>
          <span class="text-emerald-400 font-bold text-sm">{{ formatMinutes(group.total) }} total</span>
        </div>

        <!-- Project rows -->
        <table class="w-full text-sm">
          <tbody>
            <tr
              v-for="row in group.projects"
              :key="row.project_id || 'no-project'"
              class="border-b border-slate-800/40 hover:bg-slate-800/20 transition-colors"
            >
              <td class="px-5 py-3">
                <span v-if="row.project_name" class="inline-flex items-center gap-2">
                  <span class="w-2.5 h-2.5 rounded-full flex-shrink-0" :style="{ background: row.project_color || '#64748b' }"></span>
                  <span class="text-slate-300">{{ row.project_name }}</span>
                </span>
                <span v-else class="text-slate-600 italic">No Project</span>
              </td>
              <td class="px-5 py-3 text-right">
                <!-- Progress bar -->
                <div class="flex items-center gap-3 justify-end">
                  <div class="w-32 h-1.5 bg-slate-800 rounded-full overflow-hidden">
                    <div
                      class="h-full rounded-full bg-emerald-500 transition-all"
                      :style="{ width: `${Math.min(100, (row.total_minutes / group.total) * 100)}%` }"
                    ></div>
                  </div>
                  <span class="text-slate-300 font-semibold text-xs w-16 text-right">{{ formatMinutes(row.total_minutes) }}</span>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- Footer -->
    <div class="mt-3 text-xs text-slate-600 text-right">
      Showing {{ filteredRows.length }} entries across {{ groupedRows.length }} users
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useAuthStore } from '~/stores/auth'

const auth = useAuthStore()
const rows = ref([])
const loading = ref(true)
const filterUser = ref('')

onMounted(async () => {
  if (!auth.isAdmin) {
    navigateTo('/')
    return
  }
  await fetchReport()
})

async function fetchReport() {
  loading.value = true
  const { data, error } = await useFetch('/api/admin/time-report', {
    headers: { Authorization: `Bearer ${auth.token}` }
  })
  if (data.value) rows.value = data.value
  if (error.value) console.error('Failed to fetch time report', error.value)
  loading.value = false
}

// ── Computed ──
const uniqueUsers = computed(() => {
  const seen = new Map()
  for (const r of rows.value) {
    if (!seen.has(r.user_id)) {
      seen.set(r.user_id, {
        id: r.user_id,
        display: r.full_name ? `${r.full_name} (@${r.username})` : r.username
      })
    }
  }
  return [...seen.values()]
})

const filteredRows = computed(() => {
  if (!filterUser.value) return rows.value
  return rows.value.filter(r => r.user_id === filterUser.value)
})

const groupedRows = computed(() => {
  const map = new Map()
  for (const r of filteredRows.value) {
    if (!map.has(r.user_id)) {
      map.set(r.user_id, {
        user_id: r.user_id,
        username: r.username,
        display_name: r.full_name || r.username,
        total: 0,
        projects: []
      })
    }
    const group = map.get(r.user_id)
    group.total += r.total_minutes
    group.projects.push(r)
  }
  // Sort by total desc
  return [...map.values()].sort((a, b) => b.total - a.total)
})

const totalHours = computed(() => {
  const mins = rows.value.reduce((acc, r) => acc + (r.total_minutes || 0), 0)
  return (mins / 60).toFixed(1)
})

const mostActiveUser = computed(() => {
  if (groupedRows.value.length === 0) return '—'
  const top = groupedRows.value[0]
  return top.display_name
})

function formatMinutes(mins) {
  if (!mins || mins === 0) return '0m'
  const h = Math.floor(mins / 60)
  const m = mins % 60
  return h > 0 ? `${h}h ${m}m` : `${m}m`
}
</script>
