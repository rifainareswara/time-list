<template>
  <div>
    <!-- Header -->
    <div class="flex justify-between items-center mb-8">
      <div>
        <h2 class="text-3xl font-bold text-white">Projects</h2>
        <p class="text-slate-400 mt-1">Organize your work by project</p>
      </div>
      <div class="flex items-center gap-3">
        <input
          v-model="search"
          class="w-64 bg-slate-900 border border-slate-700 rounded-lg px-4 py-2 text-sm text-slate-200 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none placeholder-slate-500"
          placeholder="Search projects..."
        />
        <button class="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg font-medium shadow-sm transition-all flex items-center gap-2" @click="openCreate">
          + New Project
        </button>
      </div>
    </div>

    <!-- Project Cards Grid -->
    <div v-if="filteredProjects.length" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5">
      <div
        v-for="project in filteredProjects"
        :key="project.id"
        class="group relative bg-slate-900 rounded-xl border border-slate-800 overflow-hidden hover:border-slate-700 transition-all duration-200 hover:shadow-lg hover:shadow-black/20 hover:-translate-y-0.5 cursor-pointer"
        @click="goToTasks(project)"
      >
        <!-- Color bar -->
        <div class="h-1.5" :style="{ backgroundColor: project.color }"></div>

        <div class="p-5">
          <!-- Title row -->
          <div class="flex items-center gap-3 mb-3">
            <div class="w-4 h-4 rounded-full shrink-0 ring-2 ring-offset-2 ring-offset-slate-900" :style="{ backgroundColor: project.color, ringColor: project.color + '60' }"></div>
            <h3 class="font-bold text-lg text-white leading-tight truncate">{{ project.name }}</h3>
            <span v-if="project.id === 'default'" class="ml-auto text-[10px] font-bold uppercase tracking-wider text-slate-500 bg-slate-800 px-2 py-0.5 rounded">Default</span>
          </div>

          <!-- Description -->
          <p v-if="project.description" class="text-sm text-slate-500 leading-relaxed mb-4 line-clamp-2">{{ project.description }}</p>
          <p v-else class="text-sm text-slate-700 italic mb-4">No description</p>

          <!-- Task Stats -->
          <div class="grid grid-cols-3 gap-2 mb-4">
            <div class="bg-slate-800/60 rounded-lg px-3 py-2 text-center border border-slate-800">
              <div class="text-lg font-bold text-yellow-400">{{ project.pending_count || 0 }}</div>
              <div class="text-[10px] font-bold text-slate-500 uppercase tracking-wider">Pending</div>
            </div>
            <div class="bg-slate-800/60 rounded-lg px-3 py-2 text-center border border-slate-800">
              <div class="text-lg font-bold text-blue-400">{{ project.in_progress_count || 0 }}</div>
              <div class="text-[10px] font-bold text-slate-500 uppercase tracking-wider">Active</div>
            </div>
            <div class="bg-slate-800/60 rounded-lg px-3 py-2 text-center border border-slate-800">
              <div class="text-lg font-bold text-green-400">{{ project.completed_count || 0 }}</div>
              <div class="text-[10px] font-bold text-slate-500 uppercase tracking-wider">Done</div>
            </div>
          </div>

          <!-- Footer: total tasks + time -->
          <div class="flex items-center justify-between pt-3 border-t border-slate-800">
            <div class="flex items-center gap-2">
              <span class="text-xs font-bold text-slate-500">{{ project.task_count || 0 }} tasks</span>
              <span v-if="project.task_count > 0" class="text-slate-700">•</span>
              <span v-if="project.total_minutes" class="text-xs font-mono font-bold text-blue-400">{{ formatMin(project.total_minutes) }}</span>
            </div>

            <!-- Progress bar -->
            <div v-if="project.task_count > 0" class="flex items-center gap-2">
              <div class="w-16 h-1.5 bg-slate-800 rounded-full overflow-hidden">
                <div class="h-full bg-green-500 rounded-full transition-all" :style="{ width: completionPercent(project) + '%' }"></div>
              </div>
              <span class="text-[10px] font-bold text-slate-500">{{ completionPercent(project) }}%</span>
            </div>
          </div>
        </div>

        <!-- Hover Actions -->
        <div class="absolute top-4 right-4 flex gap-1.5 opacity-0 group-hover:opacity-100 transition-opacity" @click.stop>
          <button
            class="w-7 h-7 flex items-center justify-center rounded-md bg-slate-800 border border-slate-700 text-slate-400 hover:bg-blue-900/30 hover:text-blue-400 hover:border-blue-800 transition-colors text-xs"
            @click="openEdit(project)"
            title="Edit"
          >✏️</button>
          <button
            v-if="project.id !== 'default'"
            class="w-7 h-7 flex items-center justify-center rounded-md bg-slate-800 border border-slate-700 text-slate-400 hover:bg-red-900/30 hover:text-red-400 hover:border-red-800 transition-colors text-xs"
            @click="openDeleteConfirm(project)"
            title="Delete"
          >🗑</button>
        </div>
      </div>
    </div>

    <!-- Empty state -->
    <div v-else class="py-24 text-center">
      <div class="text-5xl mb-4 opacity-10">📁</div>
      <p class="text-slate-600">{{ search ? 'No matching projects.' : 'No projects found.' }}</p>
    </div>

    <!-- Create / Edit Project Modal -->
    <div v-if="showModal" class="fixed inset-0 bg-black/70 backdrop-blur-sm z-50 flex items-center justify-center p-4 animate-fade-in" @click.self="closeModal">
      <div class="bg-slate-900 rounded-xl shadow-2xl w-full max-w-md overflow-hidden border border-slate-800">
        <div class="px-6 py-4 border-b border-slate-800 flex justify-between items-center bg-slate-800/50">
          <h3 class="font-bold text-lg text-white">{{ editingProject ? 'Edit Project' : 'New Project' }}</h3>
          <button class="text-slate-400 hover:text-white w-8 h-8 flex items-center justify-center rounded-full hover:bg-slate-700 transition-colors" @click="closeModal">✕</button>
        </div>
        <div class="p-6 space-y-5">
          <!-- Name -->
          <div>
            <label class="block text-xs font-bold text-slate-500 uppercase tracking-wide mb-1.5">Project Name</label>
            <input v-model="form.name" class="w-full bg-slate-950 border border-slate-700 rounded-lg px-4 py-2.5 text-sm text-slate-200 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none placeholder-slate-600" placeholder="e.g. Website Redesign" />
          </div>

          <!-- Color Picker -->
          <div>
            <label class="block text-xs font-bold text-slate-500 uppercase tracking-wide mb-2">Color</label>
            <div class="flex flex-wrap gap-2">
              <button
                v-for="c in colorPalette"
                :key="c"
                class="w-8 h-8 rounded-lg transition-all border-2 hover:scale-110"
                :class="form.color === c ? 'border-white scale-110 shadow-lg' : 'border-transparent'"
                :style="{ backgroundColor: c }"
                @click="form.color = c"
              >
                <span v-if="form.color === c" class="text-white text-xs font-bold">✓</span>
              </button>
            </div>
          </div>

          <!-- Description -->
          <div>
            <label class="block text-xs font-bold text-slate-500 uppercase tracking-wide mb-1.5">Description <span class="text-slate-600 normal-case">(optional)</span></label>
            <textarea v-model="form.description" class="w-full bg-slate-950 border border-slate-700 rounded-lg px-4 py-2.5 text-sm text-slate-200 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none h-20 resize-none placeholder-slate-600" placeholder="Brief project description..."></textarea>
          </div>
        </div>
        <div class="px-6 py-4 bg-slate-800/50 flex justify-end gap-3 border-t border-slate-800">
          <button class="px-4 py-2 text-slate-400 hover:bg-slate-800 rounded-lg text-sm font-medium transition-colors" @click="closeModal">Cancel</button>
          <button class="px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-sm font-bold shadow-md hover:shadow-lg transition-all" @click="handleSave" :disabled="!form.name.trim()">Save</button>
        </div>
      </div>
    </div>

    <!-- Delete Confirmation Modal -->
    <div v-if="showDeleteModal" class="fixed inset-0 bg-black/70 backdrop-blur-sm z-50 flex items-center justify-center p-4 animate-fade-in" @click.self="showDeleteModal = false">
      <div class="bg-slate-900 rounded-xl shadow-2xl w-full max-w-sm overflow-hidden border border-slate-800">
        <div class="px-6 py-4 border-b border-slate-800 bg-slate-800/50">
          <h3 class="font-bold text-lg text-white">Delete Project</h3>
        </div>
        <div class="p-6">
          <p class="text-slate-300 text-sm">
            Are you sure you want to delete <span class="font-semibold text-red-400">{{ deleteTarget?.name }}</span>?
          </p>
          <p class="text-slate-500 text-xs mt-2">All tasks in this project will be moved to <span class="font-bold text-slate-400">General</span>.</p>
        </div>
        <div class="px-6 py-4 bg-slate-800/50 flex justify-end gap-3 border-t border-slate-800">
          <button class="px-4 py-2 text-slate-400 hover:bg-slate-800 rounded-lg text-sm font-medium transition-colors" @click="showDeleteModal = false">Cancel</button>
          <button class="px-6 py-2 bg-red-600 hover:bg-red-700 text-white rounded-lg text-sm font-bold shadow-md hover:shadow-lg transition-all" @click="confirmDelete">Delete</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useTaskStore } from '~/stores/taskStore'

const store = useTaskStore()
const router = useRouter()

const search = ref('')
const showModal = ref(false)
const showDeleteModal = ref(false)
const editingProject = ref(null)
const deleteTarget = ref(null)

const filteredProjects = computed(() => {
  if (!search.value) return store.projects
  const q = search.value.toLowerCase()
  return store.projects.filter(p =>
    p.name.toLowerCase().includes(q) || (p.description || '').toLowerCase().includes(q)
  )
})

const defaultForm = { name: '', color: '#3b82f6', description: '' }
const form = ref({ ...defaultForm })

const colorPalette = [
  '#3b82f6', '#10b981', '#f59e0b', '#ef4444',
  '#8b5cf6', '#ec4899', '#06b6d4', '#f97316',
  '#84cc16', '#6366f1', '#14b8a6', '#e11d48',
]

onMounted(() => {
  store.fetchProjects()
})

function completionPercent(project) {
  if (!project.task_count) return 0
  return Math.round(((project.completed_count || 0) / project.task_count) * 100)
}

function formatMin(m) {
  if (!m) return '0m'
  const h = Math.floor(m / 60)
  const mins = m % 60
  if (h === 0) return `${mins}m`
  if (mins === 0) return `${h}h`
  return `${h}h ${mins}m`
}

function goToTasks(project) {
  router.push({ path: '/tasks', query: { project: project.id } })
}

function openCreate() {
  editingProject.value = null
  form.value = { ...defaultForm }
  showModal.value = true
}

function openEdit(project) {
  editingProject.value = project
  form.value = {
    name: project.name,
    color: project.color,
    description: project.description || '',
  }
  showModal.value = true
}

function closeModal() {
  showModal.value = false
  editingProject.value = null
  form.value = { ...defaultForm }
}

async function handleSave() {
  if (!form.value.name.trim()) return
  try {
    if (editingProject.value) {
      await store.updateProject(editingProject.value.id, form.value)
    } else {
      await store.createProject(form.value)
    }
    closeModal()
    // Refresh to get updated stats
    await store.fetchProjects()
  } catch (e) {
    console.error('Failed to save project', e)
  }
}

function openDeleteConfirm(project) {
  deleteTarget.value = project
  showDeleteModal.value = true
}

async function confirmDelete() {
  if (!deleteTarget.value) return
  try {
    await store.deleteProject(deleteTarget.value.id)
    showDeleteModal.value = false
    deleteTarget.value = null
    await store.fetchProjects()
  } catch (e) {
    console.error('Failed to delete project', e)
  }
}
</script>
