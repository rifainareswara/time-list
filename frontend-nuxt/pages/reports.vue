<template>
  <div>
    <!-- Filters -->
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
        <div>
          <label class="block text-xs font-bold text-slate-500 uppercase tracking-wide mb-1.5">Project</label>
          <select v-model="selectedProject" @change="updatePreview" class="bg-slate-950 border border-slate-700 rounded-lg px-4 py-2.5 text-sm text-slate-200 focus:ring-2 focus:ring-blue-500 outline-none">
            <option value="">All Projects</option>
            <option v-for="p in store.projects" :key="p.id" :value="p.id">{{ p.name }}</option>
          </select>
        </div>
      </div>
    </div>

    <!-- Summary Cards -->
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

    <!-- Hours per Project Breakdown -->
    <div v-if="reportData && reportData.projectBreakdown.length" class="bg-slate-900 rounded-xl border border-slate-800 p-6 mb-6">
      <h3 class="font-bold text-slate-200 text-lg mb-5">📊 Hours per Project</h3>
      <div class="space-y-4">
        <div v-for="pb in reportData.projectBreakdown" :key="pb.name">
          <div class="flex justify-between items-center mb-1.5 text-sm">
            <div class="flex items-center gap-2">
              <span class="w-3 h-3 rounded-full shrink-0" :style="{ backgroundColor: pb.color }"></span>
              <span class="font-medium text-slate-300">{{ pb.name }}</span>
            </div>
            <span class="text-slate-400 text-xs font-mono">{{ pb.tasks }} tasks · {{ pb.hours }}h</span>
          </div>
          <div class="h-2.5 w-full bg-slate-800 rounded-full overflow-hidden">
            <div class="h-full rounded-full transition-all duration-500" :style="{ width: pb.percent + '%', backgroundColor: pb.color }"></div>
          </div>
        </div>
      </div>
    </div>

    <!-- Task Table -->
    <div v-if="reportData && reportData.tasks.length" class="bg-slate-900 rounded-xl border border-slate-800 overflow-hidden mb-6">
      <div class="px-6 py-4 border-b border-slate-800">
        <h3 class="font-bold text-slate-200">Tasks in {{ monthLabel }}</h3>
      </div>
      <table class="w-full text-sm">
        <thead>
          <tr class="border-b border-slate-800 text-slate-500 text-xs uppercase tracking-wide">
            <th class="text-left px-6 py-3">Title</th>
            <th class="text-left px-6 py-3">Project</th>
            <th class="text-left px-6 py-3">Status</th>
            <th class="text-right px-6 py-3">Hours</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="task in reportData.tasks" :key="task.id" class="border-b border-slate-800/50 hover:bg-slate-800/30">
            <td class="px-6 py-3 text-slate-200 font-medium">{{ task.title }}</td>
            <td class="px-6 py-3">
              <div class="flex items-center gap-2">
                <span class="w-2 h-2 rounded-full shrink-0" :style="{ backgroundColor: task.projectColor }"></span>
                <span class="text-slate-400">{{ task.projectName }}</span>
              </div>
            </td>
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

    <!-- Empty State -->
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
const reportData = ref(null)
const selectedProject = ref('')

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

function updatePreview() {
  reportData.value = getMonthData()
}

onMounted(async () => {
  await Promise.all([
    store.fetchTasks(),
    store.fetchAllEntries(),
    store.fetchProjects(),
  ])
  updatePreview()
})

function getProjectInfo(projectId) {
  const project = (store.projects || []).find(p => p.id === projectId)
  return project
    ? { name: project.name, color: project.color }
    : { name: 'General', color: '#3b82f6' }
}

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

  let filteredTasks = (store.tasks || []).filter(t => {
    const created = new Date(t.created_at)
    return taskMinutes[t.id] || (created >= startOfMonth && created <= endOfMonth)
  })

  // Apply project filter
  if (selectedProject.value) {
    filteredTasks = filteredTasks.filter(t => t.project_id === selectedProject.value)
  }

  const tasks = filteredTasks.map(t => {
    const info = getProjectInfo(t.project_id)
    return {
      id: t.id,
      title: t.title,
      projectName: info.name,
      projectColor: info.color,
      status: t.status,
      minutes: taskMinutes[t.id] || 0,
      hours: ((taskMinutes[t.id] || 0) / 60).toFixed(1),
    }
  })

  // Filtered entries for total count
  const filteredEntries = selectedProject.value
    ? monthEntries.filter(e => filteredTasks.some(t => t.id === e.task_id))
    : monthEntries

  const totalMinutes = filteredEntries.reduce((a, e) => a + (e.duration_minutes || 0), 0)

  // Project breakdown — aggregate hours per project
  const projectMap = {}
  tasks.forEach(t => {
    if (!projectMap[t.projectName]) {
      projectMap[t.projectName] = { name: t.projectName, color: t.projectColor, minutes: 0, tasks: 0 }
    }
    projectMap[t.projectName].minutes += t.minutes
    projectMap[t.projectName].tasks += 1
  })

  const projectBreakdown = Object.values(projectMap)
    .sort((a, b) => b.minutes - a.minutes)
    .map(pb => {
      const maxMin = Math.max(...Object.values(projectMap).map(p => p.minutes), 1)
      return {
        ...pb,
        hours: (pb.minutes / 60).toFixed(1),
        percent: (pb.minutes / maxMin) * 100,
      }
    })

  return {
    tasks,
    entries: filteredEntries,
    totalTasks: tasks.length,
    completedTasks: tasks.filter(t => t.status === 'completed').length,
    totalEntries: filteredEntries.length,
    totalHours: (totalMinutes / 60).toFixed(1),
    totalMinutes,
    projectBreakdown,
  }
}
</script>