import { defineStore } from 'pinia'

import { useAuthStore } from './auth'

// Helper for API calls
const API_BASE = '/api'
async function request(url, options = {}) {
  const auth = useAuthStore()
  const headers = {
    'Content-Type': 'application/json',
    ...(options.headers || {}),
  }

  if (auth.token) {
    headers['Authorization'] = `Bearer ${auth.token}`
  }

  const res = await fetch(`${API_BASE}${url}`, {
    ...options,
    headers,
  })
  if (!res.ok) throw new Error(`API error: ${res.status}`)
  return res.json()
}

export const api = {
  // Tasks
  getTasks: () => request('/tasks'),
  createTask: (data) => request('/tasks', { method: 'POST', body: JSON.stringify(data) }),
  updateTask: (id, data) => request(`/tasks/${id}`, { method: 'PUT', body: JSON.stringify(data) }),
  deleteTask: (id) => request(`/tasks/${id}`, { method: 'DELETE' }),

  // Time Entries
  getEntries: (taskId) => request(`/tasks/${taskId}/entries`),
  createEntry: (taskId, data) => request(`/tasks/${taskId}/entries`, { method: 'POST', body: JSON.stringify(data) }),
  deleteEntry: (id) => request(`/entries/${id}`, { method: 'DELETE' }),
  getAllEntries: () => request('/entries'),

  // Subtasks
  getSubtasks: (taskId) => request(`/tasks/${taskId}/subtasks`),
  createSubtask: (taskId, data) => request(`/tasks/${taskId}/subtasks`, { method: 'POST', body: JSON.stringify(data) }),
  updateSubtask: (id, data) => request(`/subtasks/${id}`, { method: 'PUT', body: JSON.stringify(data) }),
  deleteSubtask: (id) => request(`/subtasks/${id}`, { method: 'DELETE' }),

  // Projects
  getProjects: () => request('/projects'),
  createProject: (data) => request('/projects', { method: 'POST', body: JSON.stringify(data) }),
  updateProject: (id, data) => request(`/projects/${id}`, { method: 'PUT', body: JSON.stringify(data) }),
  deleteProject: (id) => request(`/projects/${id}`, { method: 'DELETE' }),

  // Dashboard
  getDashboard: () => request('/dashboard/summary'),

  // Timer
  startTimer: (taskId, data = {}) => request(`/timer/start/${taskId}`, { method: 'POST', body: JSON.stringify(data) }),
  stopTimer: () => request('/timer/stop', { method: 'POST' }),
  getActiveTimer: () => request('/timer/active'),
}

export const useTaskStore = defineStore('task', {
  state: () => ({
    tasks: [],
    projects: [],
    entries: [],
    allEntries: [],
    dashboard: null,
    activeTimer: null,
    timerElapsed: 0,
    loading: false,
    error: null,
    _timerInterval: null,
  }),

  getters: {
    pendingTasks: (state) => state.tasks.filter((t) => t.status === 'pending'),
    inProgressTasks: (state) => state.tasks.filter((t) => t.status === 'in_progress'),
    completedTasks: (state) => state.tasks.filter((t) => t.status === 'completed'),
    hasActiveTimer: (state) => !!state.activeTimer,
    activeTaskId: (state) => state.activeTimer?.task_id || null,
  },

  actions: {
    async fetchTasks() {
      this.loading = true
      try {
        this.tasks = await api.getTasks()
      } catch (e) {
        this.error = e.message
      } finally {
        this.loading = false
      }
    },

    async createTask(data) {
      const task = await api.createTask(data)
      this.tasks.unshift(task)
      return task
    },

    async updateTask(id, data) {
      const task = await api.updateTask(id, data)
      const idx = this.tasks.findIndex((t) => t.id === id)
      if (idx !== -1) this.tasks[idx] = task
      return task
    },

    async deleteTask(id) {
      if (!id) return
      try {
        await api.deleteTask(id)
        this.tasks = this.tasks.filter((t) => t.id !== id)
      } catch (e) {
        console.error('Failed to delete task', e)
        alert('Gagal menghapus task')
      }
    },

    async bulkDeleteTasks(ids) {
      if (!ids || !ids.length) return
      try {
        await request('/tasks/bulk-delete', { method: 'POST', body: JSON.stringify({ ids }) })
        this.tasks = this.tasks.filter((t) => !ids.includes(t.id))
      } catch (e) {
        console.error('Failed to bulk delete tasks', e)
        alert('Failed to delete selected tasks')
      }
    },

    async addTimeEntry(taskId, data) {
      const entry = await api.createEntry(taskId, data)
      await this.fetchTasks()
      return entry
    },

    async deleteTimeEntry(id) {
      await api.deleteEntry(id)
      await this.fetchTasks()
    },

    // ─── Subtasks ───

    async fetchSubtasks(taskId) {
      return await api.getSubtasks(taskId)
    },

    async addSubtask(taskId, title) {
      const subtask = await api.createSubtask(taskId, { title })
      const task = this.tasks.find((t) => t.id === taskId)
      if (task) task.subtask_count = (task.subtask_count || 0) + 1
      return subtask
    },

    async toggleSubtask(subtaskId, completed) {
      const subtask = await api.updateSubtask(subtaskId, { completed })
      const task = this.tasks.find((t) => t.id === subtask.task_id)
      if (task) {
        if (completed) {
          task.subtask_done = (task.subtask_done || 0) + 1
        } else {
          task.subtask_done = Math.max(0, (task.subtask_done || 0) - 1)
        }
      }
      return subtask
    },

    async removeSubtask(subtaskId, taskId) {
      await api.deleteSubtask(subtaskId)
      const task = this.tasks.find((t) => t.id === taskId)
      if (task) {
        task.subtask_count = Math.max(0, (task.subtask_count || 0) - 1)
      }
    },

    // ─── Projects ───

    async fetchProjects() {
      try {
        this.projects = await api.getProjects()
      } catch (e) {
        this.error = e.message
      }
    },

    async createProject(data) {
      const project = await api.createProject(data)
      this.projects.push(project)
      return project
    },

    async updateProject(id, data) {
      const project = await api.updateProject(id, data)
      const idx = this.projects.findIndex((p) => p.id === id)
      if (idx !== -1) this.projects[idx] = project
      return project
    },

    async deleteProject(id) {
      await api.deleteProject(id)
      this.projects = this.projects.filter((p) => p.id !== id)
      await this.fetchTasks()
    },

    // ─── Dashboard ───

    async fetchDashboard() {
      this.loading = true
      try {
        this.dashboard = await api.getDashboard()
      } catch (e) {
        this.error = e.message
      } finally {
        this.loading = false
      }
    },

    async fetchAllEntries() {
      this.loading = true
      try {
        this.allEntries = await api.getAllEntries()
      } catch (e) {
        this.error = e.message
      } finally {
        this.loading = false
      }
    },

    // ─── Timer ───

    async fetchActiveTimer() {
      try {
        const res = await api.getActiveTimer()
        if (res.active && res.timer) {
          this.activeTimer = res.timer
          this.timerElapsed = res.timer.elapsed_seconds
          this._startLocalTick()
        } else {
          this.activeTimer = null
          this.timerElapsed = 0
          this._stopLocalTick()
        }
      } catch (e) {
        this.activeTimer = null
      }
    },

    async startTimer(taskId, notes = '') {
      try {
        const timer = await api.startTimer(taskId, { notes })
        this.activeTimer = timer
        this.timerElapsed = 0
        this._startLocalTick()
        await this.fetchTasks()
      } catch (e) {
        this.error = e.message
      }
    },

    async stopTimer() {
      try {
        const res = await api.stopTimer()
        this.activeTimer = null
        this.timerElapsed = 0
        this._stopLocalTick()
        await this.fetchTasks()
        return res
      } catch (e) {
        this.error = e.message
      }
    },

    _startLocalTick() {
      this._stopLocalTick()
      this._timerInterval = setInterval(() => {
        this.timerElapsed++
      }, 1000)
    },

    _stopLocalTick() {
      if (this._timerInterval) {
        clearInterval(this._timerInterval)
        this._timerInterval = null
      }
    },
  },
})
