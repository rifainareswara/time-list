<template>
  <Transition name="drawer">
    <div v-if="task" class="fixed inset-0 z-50 flex justify-end" @click.self="$emit('close')">
      <!-- Backdrop -->
      <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" @click="$emit('close')"></div>

      <!-- Drawer Panel -->
      <div class="relative w-full max-w-xl bg-slate-900 border-l border-slate-800 shadow-2xl flex flex-col overflow-hidden animate-slide-in-right">
        <!-- Header -->
        <div class="flex items-center justify-between px-6 py-4 border-b border-slate-800 bg-slate-800/40">
          <div class="flex items-center gap-3">
            <div class="w-3 h-3 rounded-full" :class="statusColor"></div>
            <span class="text-xs font-bold uppercase tracking-wider text-slate-400">{{ task.status?.replace('_', ' ') }}</span>
          </div>
          <button class="w-8 h-8 flex items-center justify-center rounded-lg hover:bg-slate-700 text-slate-400 hover:text-white transition-colors" @click="$emit('close')">✕</button>
        </div>

        <!-- Content (scrollable) -->
        <div class="flex-1 overflow-y-auto p-6 space-y-6 drawer-scroll">
          <!-- Title -->
          <div>
            <h2 class="text-xl font-bold text-white leading-tight">{{ task.title }}</h2>
            <p v-if="task.description" class="text-sm text-slate-400 mt-2 leading-relaxed">{{ task.description }}</p>
            <p v-else class="text-sm text-slate-600 italic mt-2">No description</p>
          </div>

          <!-- Properties Grid -->
          <div class="grid grid-cols-2 gap-4">
            <!-- Priority -->
            <div class="bg-slate-800/50 rounded-lg p-3 border border-slate-800">
              <div class="text-[10px] font-bold text-slate-500 uppercase tracking-wider mb-2">Priority</div>
              <div class="flex gap-1.5">
                <button
                  v-for="p in priorities"
                  :key="p.value"
                  class="px-2.5 py-1 rounded-md text-[10px] font-bold uppercase tracking-wider transition-all border"
                  :class="task.priority === p.value ? p.activeClass : 'border-slate-700 text-slate-500 hover:border-slate-600'"
                  @click="updatePriority(p.value)"
                >
                  {{ p.icon }} {{ p.label }}
                </button>
              </div>
            </div>

            <!-- Category -->
            <div class="bg-slate-800/50 rounded-lg p-3 border border-slate-800">
              <div class="text-[10px] font-bold text-slate-500 uppercase tracking-wider mb-2">Category</div>
              <span class="px-2.5 py-1 text-xs font-bold rounded-md" :class="categoryClass">{{ task.category }}</span>
            </div>

            <!-- Due Date -->
            <div class="bg-slate-800/50 rounded-lg p-3 border border-slate-800">
              <div class="text-[10px] font-bold text-slate-500 uppercase tracking-wider mb-2">Due Date</div>
              <span class="text-sm text-slate-300" :class="{ 'text-red-400': isOverdue }">
                {{ task.due_date ? new Date(task.due_date).toLocaleDateString('en-US', { day: 'numeric', month: 'short', year: 'numeric' }) : '—' }}
              </span>
            </div>

            <!-- Time Tracked -->
            <div class="bg-slate-800/50 rounded-lg p-3 border border-slate-800">
              <div class="text-[10px] font-bold text-slate-500 uppercase tracking-wider mb-2">Time Tracked</div>
              <span class="text-sm font-mono font-bold text-blue-400">{{ formatMin(task.total_minutes) }}</span>
            </div>
          </div>

          <!-- Project -->
          <div class="bg-slate-800/50 rounded-lg p-3 border border-slate-800">
            <div class="text-[10px] font-bold text-slate-500 uppercase tracking-wider mb-2">Project</div>
            <select
              class="w-full bg-slate-950 border border-slate-700 rounded-lg px-3 py-1.5 text-sm text-slate-200 focus:ring-2 focus:ring-blue-500 outline-none"
              :value="task.project_id || 'default'"
              @change="updateProject($event.target.value)"
            >
              <option v-for="p in store.projects" :key="p.id" :value="p.id">{{ p.name }}</option>
            </select>
          </div>

          <!-- Status Change -->
          <div class="bg-slate-800/50 rounded-lg p-3 border border-slate-800">
            <div class="text-[10px] font-bold text-slate-500 uppercase tracking-wider mb-2">Status</div>
            <div class="flex gap-2">
              <button
                v-for="s in statuses"
                :key="s.value"
                class="flex-1 px-3 py-1.5 rounded-md text-xs font-bold transition-all border"
                :class="task.status === s.value ? s.activeClass : 'border-slate-700 text-slate-500 hover:border-slate-600'"
                @click="updateStatus(s.value)"
              >
                {{ s.label }}
              </button>
            </div>
          </div>

          <!-- Subtasks / Checklist -->
          <div class="bg-slate-800/50 rounded-lg border border-slate-800 overflow-hidden">
            <div class="px-4 py-3 border-b border-slate-800 flex items-center justify-between">
              <div class="flex items-center gap-2">
                <span class="text-sm font-bold text-slate-300">✅ Subtasks</span>
                <span v-if="subtasks.length" class="text-[10px] font-bold text-slate-500 bg-slate-700 px-1.5 py-0.5 rounded">
                  {{ completedSubtasks }}/{{ subtasks.length }}
                </span>
              </div>
            </div>

            <!-- Progress bar -->
            <div v-if="subtasks.length" class="h-1 bg-slate-800">
              <div class="h-full bg-green-500 transition-all duration-300" :style="{ width: subtaskProgress + '%' }"></div>
            </div>

            <!-- Subtask list -->
            <div class="divide-y divide-slate-800/50">
              <div
                v-for="sub in subtasks"
                :key="sub.id"
                class="group flex items-center gap-3 px-4 py-2.5 hover:bg-slate-800/30 transition-colors"
              >
                <button
                  class="w-5 h-5 rounded border-2 flex items-center justify-center transition-all shrink-0"
                  :class="sub.completed
                    ? 'bg-green-500 border-green-500 text-white'
                    : 'border-slate-600 hover:border-green-500'"
                  @click="toggleSubtask(sub)"
                >
                  <span v-if="sub.completed" class="text-[10px]">✓</span>
                </button>
                <span class="flex-1 text-sm transition-colors" :class="sub.completed ? 'text-slate-500 line-through' : 'text-slate-300'">
                  {{ sub.title }}
                </span>
                <button
                  class="opacity-0 group-hover:opacity-100 w-6 h-6 flex items-center justify-center rounded hover:bg-red-900/30 text-slate-500 hover:text-red-400 transition-all text-xs"
                  @click="removeSubtask(sub)"
                >✕</button>
              </div>
            </div>

            <!-- Add subtask -->
            <div class="px-4 py-3 border-t border-slate-800">
              <div class="flex gap-2">
                <input
                  v-model="newSubtask"
                  class="flex-1 bg-slate-950 border border-slate-700 rounded-lg px-3 py-1.5 text-sm text-slate-200 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none placeholder-slate-600"
                  placeholder="Add subtask..."
                  @keyup.enter="addSubtask"
                />
                <button
                  class="px-3 py-1.5 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-xs font-bold transition-colors"
                  :disabled="!newSubtask.trim()"
                  @click="addSubtask"
                >Add</button>
              </div>
            </div>
          </div>

          <!-- Time Entries -->
          <div class="bg-slate-800/50 rounded-lg border border-slate-800 overflow-hidden">
            <div class="px-4 py-3 border-b border-slate-800 flex items-center justify-between">
              <span class="text-sm font-bold text-slate-300">🕐 Time Entries</span>
              <span class="text-[10px] font-bold text-slate-500">{{ entries.length }} entries</span>
            </div>
            <div v-if="entries.length" class="max-h-48 overflow-y-auto divide-y divide-slate-800/50">
              <div v-for="entry in entries" :key="entry.id" class="px-4 py-2.5 flex justify-between items-center hover:bg-slate-800/30 transition-colors">
                <div>
                  <div class="text-xs text-slate-400">{{ formatDate(entry.start_time) }}</div>
                  <div v-if="entry.notes" class="text-[10px] text-slate-500 mt-0.5">{{ entry.notes }}</div>
                </div>
                <span class="text-xs font-mono font-bold text-blue-400 bg-blue-900/20 px-2 py-0.5 rounded">{{ entry.duration_minutes }}m</span>
              </div>
            </div>
            <div v-else class="px-4 py-6 text-center text-slate-600 text-sm">No time entries yet</div>
          </div>
        </div>

        <!-- Footer Actions -->
        <div class="px-6 py-4 border-t border-slate-800 bg-slate-800/40 flex items-center gap-3">
          <button
            v-if="store.activeTaskId === task.id"
            class="flex-1 px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded-lg text-sm font-bold transition-colors flex items-center justify-center gap-2"
            @click="$emit('stop-timer')"
          >
            ⏹ Stop Timer
          </button>
          <button
            v-else
            class="flex-1 px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded-lg text-sm font-bold transition-colors flex items-center justify-center gap-2"
            @click="$emit('start-timer', task)"
          >
            ▶ Start Timer
          </button>
          <button
            class="px-4 py-2 bg-slate-700 hover:bg-slate-600 text-white rounded-lg text-sm font-medium transition-colors"
            @click="$emit('log-time', task)"
          >
            🕐 Log Time
          </button>
          <button
            class="px-4 py-2 bg-slate-700 hover:bg-slate-600 text-white rounded-lg text-sm font-medium transition-colors"
            @click="$emit('edit', task)"
          >
            ✏️ Edit
          </button>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup>
import { ref, computed, watch } from 'vue'
import { useTaskStore } from '~/stores/taskStore'

const props = defineProps({
  task: { type: Object, default: null },
})

defineEmits(['close', 'edit', 'start-timer', 'stop-timer', 'log-time'])

const store = useTaskStore()
const subtasks = ref([])
const entries = ref([])
const newSubtask = ref('')

const priorities = [
  { value: 'urgent', label: 'Urgent', icon: '🔥', activeClass: 'bg-red-900/40 text-red-400 border-red-800' },
  { value: 'high', label: 'High', icon: '⬆️', activeClass: 'bg-orange-900/40 text-orange-400 border-orange-800' },
  { value: 'normal', label: 'Normal', icon: '➡️', activeClass: 'bg-blue-900/40 text-blue-400 border-blue-800' },
  { value: 'low', label: 'Low', icon: '⬇️', activeClass: 'bg-slate-700/40 text-slate-400 border-slate-600' },
]

const statuses = [
  { value: 'pending', label: 'Pending', activeClass: 'bg-yellow-900/40 text-yellow-400 border-yellow-800' },
  { value: 'in_progress', label: 'In Progress', activeClass: 'bg-blue-900/40 text-blue-400 border-blue-800' },
  { value: 'completed', label: 'Completed', activeClass: 'bg-green-900/40 text-green-400 border-green-800' },
]

const categoryColors = {
  Development: 'bg-violet-900/40 text-violet-400',
  Design: 'bg-pink-900/40 text-pink-400',
  Configuration: 'bg-cyan-900/40 text-cyan-400',
  Troubleshooting: 'bg-orange-900/40 text-orange-400',
  Meeting: 'bg-amber-900/40 text-amber-400',
  Research: 'bg-indigo-900/40 text-indigo-400',
  Testing: 'bg-lime-900/40 text-lime-400',
  Documentation: 'bg-teal-900/40 text-teal-400',
  Admin: 'bg-rose-900/40 text-rose-400',
}

const categoryClass = computed(() => categoryColors[props.task?.category] || 'bg-slate-700/40 text-slate-400')
const statusColor = computed(() => {
  if (props.task?.status === 'completed') return 'bg-green-400'
  if (props.task?.status === 'in_progress') return 'bg-blue-400'
  return 'bg-yellow-400'
})
const isOverdue = computed(() => props.task?.due_date && new Date(props.task.due_date) < new Date())
const completedSubtasks = computed(() => subtasks.value.filter(s => s.completed).length)
const subtaskProgress = computed(() => subtasks.value.length ? (completedSubtasks.value / subtasks.value.length) * 100 : 0)

// Load data when task changes
watch(() => props.task?.id, async (taskId) => {
  if (!taskId) { subtasks.value = []; entries.value = []; return }
  try {
    subtasks.value = await store.fetchSubtasks(taskId)
  } catch { subtasks.value = [] }
  try {
    const { api } = await import('~/stores/taskStore')
    entries.value = await api.getEntries(taskId)
  } catch { entries.value = [] }
}, { immediate: true })

async function updatePriority(priority) {
  if (!props.task || props.task.priority === priority) return
  await store.updateTask(props.task.id, { priority })
}

async function updateStatus(status) {
  if (!props.task || props.task.status === status) return
  await store.updateTask(props.task.id, { status })
}

async function updateProject(projectId) {
  if (!props.task || props.task.project_id === projectId) return
  await store.updateTask(props.task.id, { project_id: projectId })
}

async function addSubtask() {
  if (!newSubtask.value.trim() || !props.task) return
  const sub = await store.addSubtask(props.task.id, newSubtask.value.trim())
  subtasks.value.push(sub)
  newSubtask.value = ''
}

async function toggleSubtask(sub) {
  const updated = await store.toggleSubtask(sub.id, !sub.completed)
  const idx = subtasks.value.findIndex(s => s.id === sub.id)
  if (idx !== -1) subtasks.value[idx] = updated
}

async function removeSubtask(sub) {
  await store.removeSubtask(sub.id, props.task.id)
  subtasks.value = subtasks.value.filter(s => s.id !== sub.id)
}

function formatMin(m) {
  if (!m) return '0m'
  const h = Math.floor(m / 60)
  const mins = m % 60
  if (h === 0) return `${mins}m`
  if (mins === 0) return `${h}h`
  return `${h}h ${mins}m`
}

function formatDate(dateStr) {
  if (!dateStr) return ''
  return new Date(dateStr).toLocaleDateString('en-US', { day: 'numeric', month: 'short', hour: '2-digit', minute: '2-digit' })
}
</script>

<style scoped>
.animate-slide-in-right {
  animation: slideInRight 0.25s ease-out;
}
@keyframes slideInRight {
  from { transform: translateX(100%); opacity: 0; }
  to { transform: translateX(0); opacity: 1; }
}
.drawer-enter-active { transition: all 0.25s ease-out; }
.drawer-leave-active { transition: all 0.2s ease-in; }
.drawer-enter-from, .drawer-leave-to { opacity: 0; }
.drawer-scroll::-webkit-scrollbar { width: 4px; }
.drawer-scroll::-webkit-scrollbar-track { background: transparent; }
.drawer-scroll::-webkit-scrollbar-thumb { background: rgba(100, 116, 139, 0.3); border-radius: 4px; }
</style>
