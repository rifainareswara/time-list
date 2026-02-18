<template>
  <div class="p-6 max-w-7xl mx-auto">
    <!-- Header -->
    <div class="flex items-center justify-between mb-6">
      <div>
        <h1 class="text-2xl font-black text-white">Time Report</h1>
        <p class="text-slate-400 text-sm mt-1">Total jam kerja per user per project</p>
      </div>
      <!-- Month Picker -->
      <div class="flex items-center gap-3">
        <button @click="prevMonth" class="px-3 py-2 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-300 hover:text-white transition-colors text-sm">‹</button>
        <input
          type="month"
          v-model="selectedMonth"
          @change="fetchReport"
          class="bg-slate-800 border border-slate-700 text-white rounded-lg px-4 py-2 text-sm focus:ring-2 focus:ring-blue-500 outline-none"
        />
        <button @click="nextMonth" :disabled="selectedMonth >= currentMonth" class="px-3 py-2 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-300 hover:text-white transition-colors text-sm disabled:opacity-40 disabled:cursor-not-allowed">›</button>
      </div>
    </div>

    <!-- Summary Cards -->
    <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-8">
      <div class="bg-slate-900 border border-slate-800 rounded-xl p-4">
        <div class="text-xs text-slate-500 uppercase tracking-widest mb-1">Total Users</div>
        <div class="text-2xl font-black text-white">{{ uniqueUsers.length }}</div>
      </div>
      <div class="bg-slate-900 border border-slate-800 rounded-xl p-4">
        <div class="text-xs text-slate-500 uppercase tracking-widest mb-1">Jam Bulan Ini</div>
        <div class="text-2xl font-black text-emerald-400">{{ formatHours(grandTotalPeriod) }}</div>
      </div>
      <div class="bg-slate-900 border border-slate-800 rounded-xl p-4">
        <div class="text-xs text-slate-500 uppercase tracking-widest mb-1">Total Semua Jam</div>
        <div class="text-2xl font-black text-blue-400">{{ formatHours(grandTotalAllTime) }}</div>
      </div>
      <div class="bg-slate-900 border border-slate-800 rounded-xl p-4">
        <div class="text-xs text-slate-500 uppercase tracking-widest mb-1">Paling Aktif</div>
        <div class="text-lg font-black text-purple-400 truncate">{{ mostActiveUser }}</div>
      </div>
    </div>

    <!-- Filter by user -->
    <div class="flex items-center gap-3 mb-4">
      <select v-model="filterUser" class="bg-slate-800 border border-slate-700 text-slate-300 rounded-lg px-3 py-2 text-sm focus:ring-2 focus:ring-blue-500 outline-none">
        <option value="">Semua User</option>
        <option v-for="u in uniqueUsers" :key="u.id" :value="u.id">{{ u.full_name || u.username }}</option>
      </select>
      <span class="text-slate-500 text-sm">{{ filteredUsers.length }} user ditampilkan</span>
    </div>

    <!-- Loading -->
    <div v-if="loading" class="text-center py-16 text-slate-500">Memuat data...</div>

    <!-- Table -->
    <div v-else class="bg-slate-900 border border-slate-800 rounded-xl overflow-hidden">
      <table class="w-full text-sm">
        <thead>
          <tr class="border-b border-slate-800">
            <th class="text-left px-5 py-3 text-slate-500 font-semibold uppercase tracking-widest text-xs">User / Project</th>
            <th class="text-right px-5 py-3 text-slate-500 font-semibold uppercase tracking-widest text-xs">Jam {{ monthLabel }}</th>
            <th class="text-right px-5 py-3 text-slate-500 font-semibold uppercase tracking-widest text-xs">Total Semua</th>
          </tr>
        </thead>
        <tbody>
          <template v-for="user in filteredUsers" :key="user.id">
            <!-- User header row -->
            <tr class="border-t border-slate-800 bg-slate-800/40">
              <td class="px-5 py-3" colspan="3">
                <div class="flex items-center gap-2">
                  <div class="w-7 h-7 rounded-full bg-blue-600 flex items-center justify-center text-white text-xs font-bold">
                    {{ (user.full_name || user.username).charAt(0).toUpperCase() }}
                  </div>
                  <div>
                    <span class="text-white font-bold">{{ user.full_name || user.username }}</span>
                    <span class="text-slate-500 text-xs ml-2">@{{ user.username }}</span>
                  </div>
                </div>
              </td>
            </tr>
            <!-- Project rows -->
            <tr
              v-for="row in user.projects"
              :key="row.project_id || 'no-project'"
              class="border-t border-slate-800/50 hover:bg-slate-800/30 transition-colors"
            >
              <td class="px-5 py-2.5 pl-14">
                <div class="flex items-center gap-2">
                  <span
                    class="w-2 h-2 rounded-full flex-shrink-0"
                    :style="{ backgroundColor: row.project_color || '#64748b' }"
                  ></span>
                  <span class="text-slate-300">{{ row.project_name || 'No Project' }}</span>
                </div>
              </td>
              <td class="px-5 py-2.5 text-right">
                <span :class="row.minutes_this_period > 0 ? 'text-emerald-400 font-semibold' : 'text-slate-600'">
                  {{ formatHours(row.minutes_this_period) }}
                </span>
              </td>
              <td class="px-5 py-2.5 text-right text-slate-400">{{ formatHours(row.minutes_all_time) }}</td>
            </tr>
            <!-- User subtotal -->
            <tr class="border-t border-slate-700 bg-slate-800/20">
              <td class="px-5 py-2.5 pl-14 text-slate-500 text-xs font-semibold uppercase tracking-wide">Subtotal</td>
              <td class="px-5 py-2.5 text-right font-bold text-emerald-400">{{ formatHours(user.totalPeriod) }}</td>
              <td class="px-5 py-2.5 text-right font-bold text-blue-400">{{ formatHours(user.totalAllTime) }}</td>
            </tr>
          </template>

          <!-- Grand Total -->
          <tr v-if="filteredUsers.length > 0" class="border-t-2 border-slate-600 bg-slate-800/60">
            <td class="px-5 py-3 font-black text-white uppercase tracking-wide text-xs">Grand Total</td>
            <td class="px-5 py-3 text-right font-black text-emerald-400 text-base">{{ formatHours(grandTotalPeriod) }}</td>
            <td class="px-5 py-3 text-right font-black text-blue-400 text-base">{{ formatHours(grandTotalAllTime) }}</td>
          </tr>

          <!-- Empty -->
          <tr v-if="filteredUsers.length === 0 && !loading">
            <td colspan="3" class="px-5 py-12 text-center text-slate-500">Tidak ada data untuk periode ini.</td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Footer info -->
    <div class="mt-3 text-right text-xs text-slate-600">
      Menampilkan {{ filteredUsers.length }} user · {{ totalProjectRows }} entri project
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useAuthStore } from '~/stores/auth'

definePageMeta({ middleware: 'auth' })

const auth = useAuthStore()

// Current month as YYYY-MM
const now = new Date()
const currentMonth = `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}`
const selectedMonth = ref(currentMonth)
const filterUser = ref('')
const loading = ref(false)
const rows = ref([])

const monthLabel = computed(() => {
  const [y, m] = selectedMonth.value.split('-')
  return new Date(+y, +m - 1, 1).toLocaleString('id-ID', { month: 'long', year: 'numeric' })
})

function prevMonth() {
  const [y, m] = selectedMonth.value.split('-').map(Number)
  const d = new Date(y, m - 2, 1)
  selectedMonth.value = `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}`
  fetchReport()
}
function nextMonth() {
  const [y, m] = selectedMonth.value.split('-').map(Number)
  const d = new Date(y, m, 1)
  selectedMonth.value = `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}`
  fetchReport()
}

async function fetchReport() {
  loading.value = true
  try {
    const token = auth.token
    const res = await fetch(`/api/admin/time-report?month=${selectedMonth.value}`, {
      headers: { Authorization: `Bearer ${token}` }
    })
    const data = await res.json()
    rows.value = data.rows || []
  } catch (e) {
    rows.value = []
  } finally {
    loading.value = false
  }
}

// Group rows by user
const uniqueUsers = computed(() => {
  const map = new Map()
  for (const row of rows.value) {
    if (!map.has(row.user_id)) {
      map.set(row.user_id, {
        id: row.user_id,
        username: row.username,
        full_name: row.full_name,
        projects: [],
        totalPeriod: 0,
        totalAllTime: 0,
      })
    }
    const user = map.get(row.user_id)
    // Only add project rows that have actual data or a project
    if (row.project_id || row.minutes_all_time > 0) {
      user.projects.push(row)
    }
    user.totalPeriod += row.minutes_this_period
    user.totalAllTime += row.minutes_all_time
  }
  return [...map.values()]
})

const filteredUsers = computed(() => {
  if (!filterUser.value) return uniqueUsers.value
  return uniqueUsers.value.filter(u => u.id === filterUser.value)
})

const grandTotalPeriod = computed(() =>
  filteredUsers.value.reduce((s, u) => s + u.totalPeriod, 0)
)
const grandTotalAllTime = computed(() =>
  filteredUsers.value.reduce((s, u) => s + u.totalAllTime, 0)
)
const mostActiveUser = computed(() => {
  if (!uniqueUsers.value.length) return '—'
  const top = [...uniqueUsers.value].sort((a, b) => b.totalPeriod - a.totalPeriod)[0]
  return top.full_name || top.username
})
const totalProjectRows = computed(() =>
  filteredUsers.value.reduce((s, u) => s + u.projects.length, 0)
)

function formatHours(minutes) {
  if (!minutes) return '0h'
  const h = Math.floor(minutes / 60)
  const m = minutes % 60
  if (m === 0) return `${h}h`
  return `${h}h ${m}m`
}

onMounted(fetchReport)
</script>
