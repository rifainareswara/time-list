<template>
  <div>
    <div class="bg-slate-900 rounded-xl border border-slate-800 p-6 mb-6">
      <div class="flex flex-wrap items-end gap-4">
        <div>
          <label class="block text-xs font-bold text-slate-500 uppercase tracking-wide mb-1.5">Month</label>
          <select v-model="selectedMonth" @change="updatePreview" class="bg-slate-950 border border-slate-700 rounded-lg px-4 py-2.5 text-sm text-slate-200 focus:ring-2 focus:ring-blue-500 outline-none">
            <option v-for="m in months" :key="m.value" :value="m.value">{{ m.label }}</option>
          </select>
        </div>
        <div>
          <label class="block text-xs font-bold text-slate-500 uppercase tracking-wide mb-1.5">Year</label>
          <select v-model="selectedYear" @change="updatePreview" class="bg-slate-950 border border-slate-700 rounded-lg px-4 py-2.5 text-sm text-slate-200 focus:ring-2 focus:ring-blue-500 outline-none">
            <option v-for="y in years" :key="y" :value="y">{{ y }}</option>
          </select>
        </div>
        <!-- <button
          class="px-6 py-2.5 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-sm font-bold shadow-md hover:shadow-lg transition-all flex items-center gap-2"
          :disabled="generating"
          @click="generateReport"
        >
          <span v-if="generating">⏳ Generating...</span>
          <span v-else>📄 Generate PDF</span>
        </button> -->
      </div>
    </div>

    <div v-if="reportData" class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
      <div class="bg-slate-900 rounded-xl border border-slate-800 p-5">
        <div class="text-xs font-bold text-slate-500 uppercase tracking-wide">Total Tasks</div>
        <div class="text-2xl font-black text-white mt-1">{{ reportData.totalTasks }}</div>
      </div>
      <div class="bg-slate-900 rounded-xl border border-slate-800 p-5">
        <div class="text-xs font-bold text-slate-500 uppercase tracking-wide">Completed</div>
        <div class="text-2xl font-black text-emerald-400 mt-1">{{ reportData.completedTasks }}</div>
      </div>
      <div class="bg-slate-900 rounded-xl border border-slate-800 p-5">
        <div class="text-xs font-bold text-slate-500 uppercase tracking-wide">Time Entries</div>
        <div class="text-2xl font-black text-blue-400 mt-1">{{ reportData.totalEntries }}</div>
      </div>
      <div class="bg-slate-900 rounded-xl border border-slate-800 p-5">
        <div class="text-xs font-bold text-slate-500 uppercase tracking-wide">Total Hours</div>
        <div class="text-2xl font-black text-amber-400 mt-1">{{ reportData.totalHours }}</div>
      </div>
    </div>

    <div v-if="reportData && reportData.tasks.length" class="bg-slate-900 rounded-xl border border-slate-800 overflow-hidden mb-6">
      <div class="px-6 py-4 border-b border-slate-800">
        <h3 class="font-bold text-slate-200">Tasks in {{ monthLabel }}</h3>
      </div>
      <table class="w-full text-sm">
        <thead>
          <tr class="border-b border-slate-800 text-slate-500 text-xs uppercase tracking-wide">
            <th class="text-left px-6 py-3">Title</th>
            <th class="text-left px-6 py-3">Category</th>
            <th class="text-left px-6 py-3">Status</th>
            <th class="text-right px-6 py-3">Hours</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="task in reportData.tasks" :key="task.id" class="border-b border-slate-800/50 hover:bg-slate-800/30">
            <td class="px-6 py-3 text-slate-200 font-medium">{{ task.title }}</td>
            <td class="px-6 py-3 text-slate-400">{{ task.category }}</td>
            <td class="px-6 py-3">
              <span
                class="px-2 py-0.5 rounded text-xs font-bold"
                :class="{
                  'bg-yellow-900/30 text-yellow-400': task.status === 'pending',
                  'bg-blue-900/30 text-blue-400': task.status === 'in_progress',
                  'bg-emerald-900/30 text-emerald-400': task.status === 'completed',
                }"
              >{{ task.status }}</span>
            </td>
            <td class="px-6 py-3 text-right text-slate-300 font-mono">{{ task.hours }}</td>
          </tr>
        </tbody>
      </table>
    </div>

    <div v-if="reportData && !reportData.tasks.length" class="bg-slate-900 rounded-xl border border-slate-800 p-12 text-center">
      <div class="text-4xl mb-3 opacity-10">📋</div>
      <p class="text-slate-600">No data for {{ monthLabel }}.</p>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useTaskStore } from '~/stores/taskStore'

const store = useTaskStore()
const generating = ref(false)
const reportData = ref(null)

const now = new Date()
const selectedMonth = ref(now.getMonth())
const selectedYear = ref(now.getFullYear())

const months = [
  { value: 0, label: 'January' }, { value: 1, label: 'February' },
  { value: 2, label: 'March' }, { value: 3, label: 'April' },
  { value: 4, label: 'May' }, { value: 5, label: 'June' },
  { value: 6, label: 'July' }, { value: 7, label: 'August' },
  { value: 8, label: 'September' }, { value: 9, label: 'October' },
  { value: 10, label: 'November' }, { value: 11, label: 'December' },
]

const years = computed(() => {
  const cur = now.getFullYear()
  return [cur - 2, cur - 1, cur, cur + 1]
})

const monthLabel = computed(() => {
  return `${months[selectedMonth.value].label} ${selectedYear.value}`
})

// Fungsi untuk memperbarui preview saat bulan/tahun diubah
function updatePreview() {
  reportData.value = getMonthData()
}

onMounted(async () => {
  await store.fetchTasks()
  await store.fetchAllEntries()
  // Trigger data agar muncul saat pertama kali load
  updatePreview()
})

function getMonthData() {
  const y = selectedYear.value
  const m = selectedMonth.value
  const startOfMonth = new Date(y, m, 1)
  const endOfMonth = new Date(y, m + 1, 0, 23, 59, 59)

  const monthEntries = (store.allEntries || []).filter(e => {
    const d = new Date(e.start_time)
    return d >= startOfMonth && d <= endOfMonth
  })

  const taskMinutes = {}
  monthEntries.forEach(e => {
    const tid = e.task_id
    taskMinutes[tid] = (taskMinutes[tid] || 0) + (e.duration_minutes || 0)
  })

  const tasks = (store.tasks || []).filter(t => {
    const created = new Date(t.created_at)
    return taskMinutes[t.id] || (created >= startOfMonth && created <= endOfMonth)
  }).map(t => ({
    id: t.id,
    title: t.title,
    category: t.category || 'General',
    status: t.status,
    minutes: taskMinutes[t.id] || 0,
    hours: ((taskMinutes[t.id] || 0) / 60).toFixed(1),
  }))

  const totalMinutes = Object.values(taskMinutes).reduce((a, b) => a + b, 0)

  return {
    tasks,
    entries: monthEntries,
    totalTasks: tasks.length,
    completedTasks: tasks.filter(t => t.status === 'completed').length,
    totalEntries: monthEntries.length,
    totalHours: (totalMinutes / 60).toFixed(1),
    totalMinutes,
  }
}

async function generateReport() {
  generating.value = true
  await store.fetchTasks()
  await store.fetchAllEntries()
  
  const data = getMonthData()
  reportData.value = data

  const { jsPDF } = await import('jspdf')
  const autoTableModule = await import('jspdf-autotable')
  const doc = new jsPDF()
  // ... (sisa logika PDF tetap sama)
  generating.value = false
}
</script>