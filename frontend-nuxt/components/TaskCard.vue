<template>
  <div
    draggable="true"
    class="group bg-slate-800/80 hover:bg-slate-800 rounded-lg border border-slate-700/50 hover:border-slate-600 p-4 cursor-grab active:cursor-grabbing transition-all duration-200 hover:shadow-lg hover:shadow-black/20 hover:-translate-y-0.5"
    :class="[
      activeTaskId === task.id ? 'ring-2 ring-blue-500/40 border-blue-700/50' : '',
      priorityBorder
    ]"
    @click="$emit('open-detail', task)"
  >
    <!-- Priority & Category & Time -->
    <div class="flex items-center justify-between mb-2">
      <div class="flex items-center gap-1.5">
        <span class="text-xs" :title="'Priority: ' + task.priority">{{ priorityIcon }}</span>
        <span class="px-2 py-0.5 text-[10px] font-bold uppercase tracking-wider rounded-md"
          :class="categoryClass">
          {{ task.category }}
        </span>
      </div>
      <span v-if="task.total_minutes" class="text-[10px] font-mono font-bold text-slate-400">
        {{ formatMin(task.total_minutes) }}
      </span>
    </div>

    <!-- Project badge -->
    <div v-if="task.project_name" class="mb-2">
      <span class="inline-flex items-center gap-1 px-1.5 py-0.5 text-[10px] font-bold rounded" :style="{ backgroundColor: (task.project_color || '#3b82f6') + '18', color: task.project_color || '#3b82f6' }">
        <span class="w-1.5 h-1.5 rounded-full" :style="{ backgroundColor: task.project_color || '#3b82f6' }"></span>
        {{ task.project_name }}
      </span>
    </div>

    <!-- Title -->
    <h4 class="font-semibold text-sm text-slate-200 mb-1 leading-snug">{{ task.title }}</h4>

    <!-- Description -->
    <p v-if="task.description" class="text-xs text-slate-500 leading-relaxed mb-3 line-clamp-2">{{ task.description }}</p>

    <!-- Subtask progress -->
    <div v-if="task.subtask_count > 0" class="flex items-center gap-2 mb-3">
      <div class="flex-1 h-1.5 bg-slate-700 rounded-full overflow-hidden">
        <div class="h-full bg-green-500 rounded-full transition-all duration-300" :style="{ width: subtaskProgress + '%' }"></div>
      </div>
      <span class="text-[10px] font-bold text-slate-400">{{ task.subtask_done || 0 }}/{{ task.subtask_count }}</span>
    </div>

    <!-- Due date -->
    <div v-if="task.due_date" class="flex items-center gap-1 mb-3">
      <span class="text-[10px] text-slate-500">📅</span>
      <span class="text-[10px] font-medium"
        :class="isOverdue ? 'text-red-400' : 'text-slate-400'">
        {{ formattedDate }}
      </span>
    </div>

    <!-- Actions -->
    <div class="flex items-center gap-1.5 pt-2 border-t border-slate-700/30 opacity-0 group-hover:opacity-100 transition-opacity">
      <button
        v-if="activeTaskId === task.id"
        class="flex-1 px-2 py-1 bg-red-900/30 text-red-400 border border-red-900/40 rounded text-[10px] font-bold transition-colors hover:bg-red-900/50"
        @click.stop="$emit('stop-timer')"
      >
        ⏹ {{ formattedElapsed }}
      </button>
      <button
        v-else
        class="px-2 py-1 bg-green-900/20 text-green-400 border border-green-900/30 rounded text-[10px] font-bold transition-colors hover:bg-green-900/40"
        @click.stop="$emit('start-timer', task)"
        title="Start timer"
      >▶</button>
      <button
        class="px-2 py-1 bg-slate-700/30 text-slate-400 border border-slate-700/40 rounded text-[10px] font-medium transition-colors hover:bg-slate-700/60"
        @click.stop="$emit('log-time', task)"
        title="Log time"
      >🕐</button>
      <button
        class="px-2 py-1 bg-slate-700/30 text-slate-400 border border-slate-700/40 rounded text-[10px] font-medium transition-colors hover:bg-blue-900/30 hover:text-blue-400"
        @click.stop="$emit('edit', task)"
        title="Edit"
      >✏️</button>
      <button
        class="px-2 py-1 bg-slate-700/30 text-slate-400 border border-slate-700/40 rounded text-[10px] font-medium transition-colors hover:bg-red-900/30 hover:text-red-400"
        @click.stop="$emit('delete', task)"
        title="Delete"
      >🗑</button>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'

const props = defineProps({
  task: { type: Object, required: true },
  activeTaskId: { type: String, default: null },
  timerElapsed: { type: Number, default: 0 },
})

defineEmits(['start-timer', 'stop-timer', 'edit', 'delete', 'log-time', 'open-detail'])

const priorityIcons = { urgent: '🔥', high: '⬆️', normal: '➡️', low: '⬇️' }
const priorityBorders = {
  urgent: 'border-l-4 !border-l-red-500',
  high: 'border-l-4 !border-l-orange-500',
  normal: '',
  low: 'border-l-4 !border-l-slate-600',
}

const priorityIcon = computed(() => priorityIcons[props.task.priority] || '➡️')
const priorityBorder = computed(() => priorityBorders[props.task.priority] || '')

const categoryColors = {
  Development: 'bg-violet-900/40 text-violet-400 border border-violet-800/40',
  Design: 'bg-pink-900/40 text-pink-400 border border-pink-800/40',
  Configuration: 'bg-cyan-900/40 text-cyan-400 border border-cyan-800/40',
  Troubleshooting: 'bg-orange-900/40 text-orange-400 border border-orange-800/40',
  Meeting: 'bg-amber-900/40 text-amber-400 border border-amber-800/40',
  Research: 'bg-indigo-900/40 text-indigo-400 border border-indigo-800/40',
  Testing: 'bg-lime-900/40 text-lime-400 border border-lime-800/40',
  Documentation: 'bg-teal-900/40 text-teal-400 border border-teal-800/40',
  Admin: 'bg-rose-900/40 text-rose-400 border border-rose-800/40',
}

const categoryClass = computed(() => categoryColors[props.task.category] || 'bg-slate-700/40 text-slate-400 border border-slate-600/40')

const isOverdue = computed(() => {
  if (!props.task.due_date) return false
  return new Date(props.task.due_date) < new Date()
})

const formattedDate = computed(() => {
  if (!props.task.due_date) return ''
  return new Date(props.task.due_date).toLocaleDateString('en-US', { day: 'numeric', month: 'short' })
})

const subtaskProgress = computed(() => {
  if (!props.task.subtask_count) return 0
  return ((props.task.subtask_done || 0) / props.task.subtask_count) * 100
})

const formattedElapsed = computed(() => {
  const m = Math.floor((props.timerElapsed % 3600) / 60)
  const s = props.timerElapsed % 60
  const pad = (n) => String(n).padStart(2, '0')
  return `${pad(m)}:${pad(s)}`
})

function formatMin(m) {
  if (!m) return '0m'
  const h = Math.floor(m / 60)
  const mins = m % 60
  if (h === 0) return `${mins}m`
  if (mins === 0) return `${h}h`
  return `${h}h ${mins}m`
}
</script>
