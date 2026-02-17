<template>
  <div>
    <!-- Active Timer Bar -->
    <div v-if="store.activeTimer" class="fixed top-0 left-0 right-0 h-14 bg-slate-900 z-40 flex items-center justify-center text-white shadow-lg animate-slide-down border-b border-slate-800">
      <div class="flex items-center gap-6 text-sm">
        <div class="flex items-center gap-2">
          <div class="w-2 h-2 rounded-full bg-red-500 animate-pulse"></div>
          <span class="text-slate-400 font-medium tracking-wide text-xs uppercase">Timer Active</span>
          <span class="font-bold text-white">{{ store.activeTimer.task_title }}</span>
        </div>
        <div class="flex items-center gap-4">
          <span class="font-mono text-xl font-bold tracking-wider text-green-400">{{ formatElapsed(store.timerElapsed) }}</span>
          <button class="bg-red-500 hover:bg-red-600 text-white px-3 py-1 rounded text-xs font-bold transition-colors" @click="handleStopTimer">Stop</button>
        </div>
      </div>
    </div>

    <!-- Header -->
    <div class="flex justify-between items-center mb-8" :class="{ 'mt-10': store.activeTimer }">
      <div>
        <h2 class="text-3xl font-bold text-white">Tasks</h2>
        <p class="text-slate-400 mt-1">Manage your work list and track time</p>
      </div>
      <div class="flex items-center gap-3">
        <!-- View Mode Toggle -->
        <div class="flex bg-slate-800 rounded-lg p-1 border border-slate-700">
          <button
            class="px-3 py-1.5 rounded-md text-xs font-bold transition-all"
            :class="viewMode === 'list' ? 'bg-blue-600 text-white shadow-md' : 'text-slate-400 hover:text-slate-200'"
            @click="viewMode = 'list'"
          >📋 List</button>
          <button
            class="px-3 py-1.5 rounded-md text-xs font-bold transition-all"
            :class="viewMode === 'board' ? 'bg-blue-600 text-white shadow-md' : 'text-slate-400 hover:text-slate-200'"
            @click="viewMode = 'board'"
          >📊 Board</button>
        </div>
        <button class="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg font-medium shadow-sm transition-all flex items-center gap-2" @click="showCreateModal = true">
          New Task
        </button>
      </div>
    </div>

    <!-- Filter -->
    <div class="flex flex-wrap gap-4 mb-6 bg-slate-900 p-4 rounded-xl border border-slate-800 shadow-sm">
      <input
        v-model="search"
        class="flex-1 min-w-[200px] bg-slate-950 border border-slate-700 rounded-lg px-4 py-2 text-sm text-slate-200 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-all placeholder-slate-500"
        placeholder="Search tasks..."
      />
      <select v-if="viewMode === 'list'" v-model="filterStatus" class="w-40 bg-slate-950 border border-slate-700 rounded-lg px-4 py-2 text-sm text-slate-200 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none">
        <option value="">All Status</option>
        <option value="pending">Pending</option>
        <option value="in_progress">In Progress</option>
        <option value="completed">Completed</option>
      </select>
      <select v-model="filterPriority" class="w-40 bg-slate-950 border border-slate-700 rounded-lg px-4 py-2 text-sm text-slate-200 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none">
        <option value="">All Priority</option>
        <option value="urgent">🔥 Urgent</option>
        <option value="high">⬆️ High</option>
        <option value="normal">➡️ Normal</option>
        <option value="low">⬇️ Low</option>
      </select>
      <select v-model="filterProject" class="w-44 bg-slate-950 border border-slate-700 rounded-lg px-4 py-2 text-sm text-slate-200 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none">
        <option value="">All Projects</option>
        <option v-for="p in store.projects" :key="p.id" :value="p.id">{{ p.name }}</option>
      </select>
      <input v-model="filterStartDate" type="date" class="bg-slate-950 border border-slate-700 rounded-lg px-4 py-2 text-sm text-slate-200 focus:ring-2 focus:ring-blue-500 outline-none" title="Filter From" />
      <input v-model="filterEndDate" type="date" class="bg-slate-950 border border-slate-700 rounded-lg px-4 py-2 text-sm text-slate-200 focus:ring-2 focus:ring-blue-500 outline-none" title="Filter To" />
    </div>

    <!-- Bulk Actions (List only) -->
    <div v-if="viewMode === 'list' && selectedTaskIds.length" class="mb-4 p-3 bg-red-900/20 border border-red-900/30 rounded-lg flex items-center justify-between">
      <div class="flex items-center gap-2 text-red-400">
        <span class="font-bold">{{ selectedTaskIds.length }}</span>
        <span class="text-sm font-medium">tasks selected</span>
      </div>
      <button class="bg-slate-800 text-red-400 border border-slate-700 hover:bg-slate-700 px-3 py-1.5 rounded text-sm font-medium transition-colors" @click="handleBulkDelete">
        Delete Selected
      </button>
    </div>

    <!-- ==================== LIST VIEW ==================== -->
    <div v-if="viewMode === 'list'" class="bg-slate-900 rounded-xl border border-slate-800 shadow-sm overflow-hidden">
      <div class="overflow-x-auto">
        <table v-if="filteredTasks.length" class="w-full text-left border-collapse">
          <thead>
            <tr class="bg-slate-800/50 border-b border-slate-800 text-xs text-slate-400 uppercase tracking-wider font-semibold">
              <th class="p-4 w-10 text-center"><input type="checkbox" :checked="isAllSelected" @change="toggleSelectAll" class="rounded border-slate-600 text-blue-600 focus:ring-blue-500 bg-slate-700" /></th>
              <th class="p-4 w-10">P</th>
              <th class="p-4">Task</th>
              <th class="p-4">Project</th>
              <th class="p-4">Category</th>
              <th class="p-4">Dates</th>
              <th class="p-4">Status</th>
              <th class="p-4">Time</th>
              <th class="p-4">Entries</th>
              <th class="p-4 w-60"></th>
            </tr>
          </thead>
          <tbody class="divide-y divide-slate-800">
            <tr v-for="task in filteredTasks" :key="task.id" 
                class="hover:bg-slate-800/50 transition-colors group cursor-pointer"
                :class="{ 'bg-blue-900/10': store.activeTaskId === task.id }"
                @click="openDetail(task)">
              <td class="p-4 text-center" @click.stop><input type="checkbox" :value="task.id" v-model="selectedTaskIds" class="rounded border-slate-600 text-blue-600 focus:ring-blue-500 bg-slate-700" /></td>
              <td class="p-4" @click.stop>
                <span class="text-base" :title="'Priority: ' + (task.priority || 'normal')">{{ priorityIcon(task.priority) }}</span>
              </td>
              <td class="p-4">
                <div class="font-bold text-slate-200">{{ task.title }}</div>
                <div class="text-xs text-slate-500 truncate mt-0.5 max-w-[280px]">{{ task.description || '—' }}</div>
                <div v-if="task.subtask_count > 0" class="flex items-center gap-2 mt-1.5">
                  <div class="w-20 h-1 bg-slate-700 rounded-full overflow-hidden">
                    <div class="h-full bg-green-500 rounded-full transition-all" :style="{ width: subtaskPercent(task) + '%' }"></div>
                  </div>
                  <span class="text-[10px] font-bold text-slate-500">{{ task.subtask_done || 0 }}/{{ task.subtask_count }}</span>
                </div>
              </td>
              <td class="p-4">
                <span v-if="task.project_name" class="inline-flex items-center gap-1.5 px-2 py-1 text-xs rounded-full font-bold border border-slate-700/50" :style="{ backgroundColor: (task.project_color || '#3b82f6') + '18', color: task.project_color || '#3b82f6', borderColor: (task.project_color || '#3b82f6') + '40' }">
                  <span class="w-1.5 h-1.5 rounded-full" :style="{ backgroundColor: task.project_color || '#3b82f6' }"></span>
                  {{ task.project_name }}
                </span>
                <span v-else class="text-slate-700 text-xs">—</span>
              </td>
              <td class="p-4"><span class="px-2 py-1 bg-slate-800 text-slate-400 text-xs rounded-full font-medium border border-slate-700">{{ task.category }}</span></td>
              <td class="p-4">
                <div v-if="task.start_date" class="text-xs text-slate-500">Start: {{ formatDate(task.start_date) }}</div>
                <div v-if="task.due_date" class="text-xs text-slate-400 font-medium">Due: {{ formatDate(task.due_date) }}</div>
                <div v-if="!task.start_date && !task.due_date" class="text-slate-700 text-xs">—</div>
              </td>
              <td class="p-4">
                <span class="px-2.5 py-1 text-xs rounded-full font-bold inline-flex items-center gap-1.5"
                  :class="{
                    'bg-yellow-900/30 text-yellow-400 border border-yellow-900/50': task.status === 'pending',
                    'bg-blue-900/30 text-blue-400 border border-blue-900/50': task.status === 'in_progress',
                    'bg-green-900/30 text-green-400 border border-green-900/50': task.status === 'completed'
                  }">
                  {{ statusLabel(task.status) }}
                </span>
              </td>
              <td class="p-4 font-mono font-medium text-slate-300">{{ formatMin(task.total_minutes) }}</td>
              <td class="p-4 text-sm text-slate-500 pl-6">{{ task.entry_count }}</td>
              <td class="p-4 text-right" @click.stop>
                <div class="flex justify-end gap-2">
                  <button
                    v-if="store.activeTaskId === task.id"
                    class="px-3 py-1.5 bg-red-900/30 text-red-400 border border-red-900/50 rounded hover:bg-red-900/50 text-xs font-bold transition-colors"
                    @click="handleStopTimer"
                    title="Stop timer"
                  >
                    {{ formatElapsed(store.timerElapsed) }}
                  </button>
                  <button
                    v-else
                    class="px-3 py-1.5 bg-green-900/30 text-green-400 border border-green-900/50 rounded hover:bg-green-900/50 text-xs font-bold transition-colors"
                    @click="handleStartTimer(task)"
                    title="Start timer"
                  >
                    Start
                  </button>
                  <button class="px-3 py-1.5 bg-slate-800 text-slate-300 border border-slate-700 rounded hover:bg-slate-700 text-xs font-medium transition-colors" @click="openTimeEntry(task)">Log</button>
                  <button class="px-3 py-1.5 bg-slate-800 text-slate-300 border border-slate-700 rounded hover:bg-blue-900/30 hover:text-blue-400 hover:border-blue-800 text-xs font-medium transition-colors" @click="openEdit(task)">
                    Edit
                  </button>
                  <button class="px-3 py-1.5 bg-slate-800 text-slate-300 border border-slate-700 rounded hover:bg-red-900/30 hover:text-red-400 hover:border-red-800 text-xs font-medium transition-colors" @click="handleDelete(task)">
                    Delete
                  </button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
        <div v-else class="py-16 text-center">
          <div class="text-4xl mb-4 opacity-10">📋</div>
          <p class="text-slate-600">No tasks found.</p>
        </div>
      </div>
    </div>

    <!-- ==================== BOARD VIEW ==================== -->
    <div v-if="viewMode === 'board'" class="grid grid-cols-3 gap-5 min-h-[calc(100vh-280px)]">
      <!-- PENDING Column -->
      <div class="flex flex-col">
        <div class="flex items-center gap-2 mb-4 px-1">
          <div class="w-3 h-3 rounded-full bg-yellow-400"></div>
          <h3 class="font-bold text-sm uppercase tracking-wider text-slate-300">Pending</h3>
          <span class="ml-auto text-xs font-bold text-slate-500 bg-slate-800 px-2 py-0.5 rounded-full">{{ boardPending.length }}</span>
        </div>
        <div
          class="flex-1 bg-slate-900/40 rounded-xl border-2 border-dashed border-slate-800 p-3 space-y-3 transition-colors overflow-y-auto max-h-[calc(100vh-320px)]"
          :class="{ 'border-yellow-500/50 bg-yellow-900/10': dragOverColumn === 'pending' }"
          @dragover.prevent="handleDragOver('pending')"
          @dragleave="handleDragLeave"
          @drop.prevent="handleDrop('pending')"
        >
          <div v-if="!boardPending.length && dragOverColumn !== 'pending'" class="py-12 text-center">
            <div class="text-3xl mb-2 opacity-20">📋</div>
            <p class="text-slate-600 text-xs">No pending tasks</p>
          </div>
          <TaskCard
            v-for="task in boardPending" :key="task.id"
            :task="task" :active-task-id="store.activeTaskId" :timer-elapsed="store.timerElapsed"
            @dragstart="handleDragStart($event, task)" @dragend="handleDragEnd"
            @start-timer="handleStartTimer" @stop-timer="handleStopTimer"
            @edit="openEdit" @delete="handleDelete" @log-time="openTimeEntry"
            @open-detail="openDetail"
          />
        </div>
      </div>

      <!-- IN PROGRESS Column -->
      <div class="flex flex-col">
        <div class="flex items-center gap-2 mb-4 px-1">
          <div class="w-3 h-3 rounded-full bg-blue-400"></div>
          <h3 class="font-bold text-sm uppercase tracking-wider text-slate-300">In Progress</h3>
          <span class="ml-auto text-xs font-bold text-slate-500 bg-slate-800 px-2 py-0.5 rounded-full">{{ boardInProgress.length }}</span>
        </div>
        <div
          class="flex-1 bg-slate-900/40 rounded-xl border-2 border-dashed border-slate-800 p-3 space-y-3 transition-colors overflow-y-auto max-h-[calc(100vh-320px)]"
          :class="{ 'border-blue-500/50 bg-blue-900/10': dragOverColumn === 'in_progress' }"
          @dragover.prevent="handleDragOver('in_progress')"
          @dragleave="handleDragLeave"
          @drop.prevent="handleDrop('in_progress')"
        >
          <div v-if="!boardInProgress.length && dragOverColumn !== 'in_progress'" class="py-12 text-center">
            <div class="text-3xl mb-2 opacity-20">🚀</div>
            <p class="text-slate-600 text-xs">No tasks in progress</p>
          </div>
          <TaskCard
            v-for="task in boardInProgress" :key="task.id"
            :task="task" :active-task-id="store.activeTaskId" :timer-elapsed="store.timerElapsed"
            @dragstart="handleDragStart($event, task)" @dragend="handleDragEnd"
            @start-timer="handleStartTimer" @stop-timer="handleStopTimer"
            @edit="openEdit" @delete="handleDelete" @log-time="openTimeEntry"
            @open-detail="openDetail"
          />
        </div>
      </div>

      <!-- COMPLETED Column -->
      <div class="flex flex-col">
        <div class="flex items-center gap-2 mb-4 px-1">
          <div class="w-3 h-3 rounded-full bg-green-400"></div>
          <h3 class="font-bold text-sm uppercase tracking-wider text-slate-300">Completed</h3>
          <span class="ml-auto text-xs font-bold text-slate-500 bg-slate-800 px-2 py-0.5 rounded-full">{{ boardCompleted.length }}</span>
        </div>
        <div
          class="flex-1 bg-slate-900/40 rounded-xl border-2 border-dashed border-slate-800 p-3 space-y-3 transition-colors overflow-y-auto max-h-[calc(100vh-320px)]"
          :class="{ 'border-green-500/50 bg-green-900/10': dragOverColumn === 'completed' }"
          @dragover.prevent="handleDragOver('completed')"
          @dragleave="handleDragLeave"
          @drop.prevent="handleDrop('completed')"
        >
          <div v-if="!boardCompleted.length && dragOverColumn !== 'completed'" class="py-12 text-center">
            <div class="text-3xl mb-2 opacity-20">✅</div>
            <p class="text-slate-600 text-xs">No completed tasks</p>
          </div>
          <TaskCard
            v-for="task in boardCompleted" :key="task.id"
            :task="task" :active-task-id="store.activeTaskId" :timer-elapsed="store.timerElapsed"
            @dragstart="handleDragStart($event, task)" @dragend="handleDragEnd"
            @start-timer="handleStartTimer" @stop-timer="handleStopTimer"
            @edit="openEdit" @delete="handleDelete" @log-time="openTimeEntry"
            @open-detail="openDetail"
          />
        </div>
      </div>
    </div>

    <!-- Task Detail Drawer -->
    <TaskDetailDrawer
      :task="detailTask"
      @close="detailTask = null"
      @edit="openEditFromDrawer"
      @start-timer="handleStartTimer"
      @stop-timer="handleStopTimer"
      @log-time="openTimeEntry"
    />

    <!-- Create / Edit Task Modal -->
    <div v-if="showCreateModal || editingTask" class="fixed inset-0 bg-black/70 backdrop-blur-sm z-50 flex items-center justify-center p-4 animate-fade-in" @click.self="closeModals">
      <div class="bg-slate-900 rounded-xl shadow-2xl w-full max-w-lg overflow-hidden transform transition-all scale-100 border border-slate-800">
        <div class="px-6 py-4 border-b border-slate-800 flex justify-between items-center bg-slate-800/50">
          <h3 class="font-bold text-lg text-white">{{ editingTask ? 'Edit Task' : 'New Task' }}</h3>
          <button class="text-slate-400 hover:text-white w-8 h-8 flex items-center justify-center rounded-full hover:bg-slate-700 transition-colors" @click="closeModals">✕</button>
        </div>
        <div class="p-6 space-y-4">
          <div>
            <label class="block text-xs font-bold text-slate-500 uppercase tracking-wide mb-1.5">Title</label>
            <input v-model="form.title" class="w-full bg-slate-950 border border-slate-700 rounded-lg px-4 py-2.5 text-sm text-slate-200 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none placeholder-slate-600" placeholder="Task name..." />
          </div>
          <div>
            <label class="block text-xs font-bold text-slate-500 uppercase tracking-wide mb-1.5">Description</label>
            <textarea v-model="form.description" class="w-full bg-slate-950 border border-slate-700 rounded-lg px-4 py-2.5 text-sm text-slate-200 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none h-24 resize-none placeholder-slate-600" placeholder="Work description..."></textarea>
          </div>
          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-xs font-bold text-slate-500 uppercase tracking-wide mb-1.5">Category</label>
              <select v-model="form.category" class="w-full bg-slate-950 border border-slate-700 rounded-lg px-4 py-2.5 text-sm text-slate-200 focus:ring-2 focus:ring-blue-500 outline-none">
                <option value="General">General</option>
                <option value="Development">Development</option>
                <option value="Configuration">Configuration</option>
                <option value="Troubleshooting">Troubleshooting</option>
                <option value="Design">Design</option>
                <option value="Meeting">Meeting</option>
                <option value="Research">Research</option>
                <option value="Admin">Admin</option>
                <option value="Testing">Testing</option>
                <option value="Documentation">Documentation</option>
              </select>
            </div>
            <div>
              <label class="block text-xs font-bold text-slate-500 uppercase tracking-wide mb-1.5">Priority</label>
              <select v-model="form.priority" class="w-full bg-slate-950 border border-slate-700 rounded-lg px-4 py-2.5 text-sm text-slate-200 focus:ring-2 focus:ring-blue-500 outline-none">
                <option value="urgent">🔥 Urgent</option>
                <option value="high">⬆️ High</option>
                <option value="normal">➡️ Normal</option>
                <option value="low">⬇️ Low</option>
              </select>
            </div>
          </div>
          <div>
            <label class="block text-xs font-bold text-slate-500 uppercase tracking-wide mb-1.5">Project</label>
            <select v-model="form.project_id" class="w-full bg-slate-950 border border-slate-700 rounded-lg px-4 py-2.5 text-sm text-slate-200 focus:ring-2 focus:ring-blue-500 outline-none">
              <option v-for="p in store.projects" :key="p.id" :value="p.id">{{ p.name }}</option>
            </select>
          </div>
          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-xs font-bold text-slate-500 uppercase tracking-wide mb-1.5">Start Date</label>
              <input v-model="form.start_date" type="date" class="w-full bg-slate-950 border border-slate-700 rounded-lg px-4 py-2.5 text-sm text-slate-200 focus:ring-2 focus:ring-blue-500 outline-none" />
            </div>
            <div>
              <label class="block text-xs font-bold text-slate-500 uppercase tracking-wide mb-1.5">Due Date</label>
              <input v-model="form.due_date" type="date" class="w-full bg-slate-950 border border-slate-700 rounded-lg px-4 py-2.5 text-sm text-slate-200 focus:ring-2 focus:ring-blue-500 outline-none" />
            </div>
          </div>
          <div v-if="editingTask">
            <label class="block text-xs font-bold text-slate-500 uppercase tracking-wide mb-1.5">Status</label>
            <select v-model="form.status" class="w-full bg-slate-950 border border-slate-700 rounded-lg px-4 py-2.5 text-sm text-slate-200 focus:ring-2 focus:ring-blue-500 outline-none">
              <option value="pending">Pending</option>
              <option value="in_progress">In Progress</option>
              <option value="completed">Completed</option>
            </select>
          </div>
        </div>
        <div class="px-6 py-4 bg-slate-800/50 flex justify-end gap-3 border-t border-slate-800">
          <button class="px-4 py-2 text-slate-400 hover:bg-slate-800 rounded-lg text-sm font-medium transition-colors" @click="closeModals">Cancel</button>
          <button class="px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-sm font-bold shadow-md hover:shadow-lg transition-all" @click="handleSave">Save</button>
        </div>
      </div>
    </div>

    <!-- Time Entry Modal -->
    <div v-if="showTimeModal" class="fixed inset-0 bg-black/70 backdrop-blur-sm z-50 flex items-center justify-center p-4 animate-fade-in" @click.self="showTimeModal = false">
      <div class="bg-slate-900 rounded-xl shadow-2xl w-full max-w-lg overflow-hidden border border-slate-800">
        <div class="px-6 py-4 border-b border-slate-800 flex justify-between items-center bg-slate-800/50">
          <h3 class="font-bold text-lg text-white">Log Time — <span class="text-blue-500">{{ selectedTask?.title }}</span></h3>
          <button class="text-slate-400 hover:text-white w-8 h-8 flex items-center justify-center rounded-full hover:bg-slate-700 transition-colors" @click="showTimeModal = false">✕</button>
        </div>
        <div class="p-6 space-y-4">
          <div>
            <label class="block text-xs font-bold text-slate-500 uppercase tracking-wide mb-1.5">Duration (minutes)</label>
            <input v-model.number="timeForm.duration_minutes" type="number" min="1" class="w-full bg-slate-950 border border-slate-700 rounded-lg px-4 py-2.5 text-sm text-slate-200 focus:ring-2 focus:ring-blue-500 outline-none" placeholder="30" />
          </div>
          <div>
            <label class="block text-xs font-bold text-slate-500 uppercase tracking-wide mb-1.5">Notes (optional)</label>
            <textarea v-model="timeForm.notes" class="w-full bg-slate-950 border border-slate-700 rounded-lg px-4 py-2.5 text-sm text-slate-200 focus:ring-2 focus:ring-blue-500 outline-none h-20 resize-none placeholder-slate-600" placeholder="What did you do..."></textarea>
          </div>
        </div>
        <div class="px-6 py-4 bg-slate-800/50 flex justify-end gap-3 border-t border-slate-800">
          <button class="px-4 py-2 text-slate-400 hover:bg-slate-800 rounded-lg text-sm font-medium transition-colors" @click="showTimeModal = false">Cancel</button>
          <button class="px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-sm font-bold shadow-md hover:shadow-lg transition-all" @click="handleTimeEntry">Save Log</button>
        </div>
      </div>
    </div>

    <!-- Delete Confirmation Modal -->
    <div v-if="showDeleteModal" class="fixed inset-0 bg-black/70 backdrop-blur-sm z-50 flex items-center justify-center p-4 animate-fade-in" @click.self="showDeleteModal = false">
      <div class="bg-slate-900 rounded-xl shadow-2xl w-full max-w-sm overflow-hidden border border-slate-800">
        <div class="px-6 py-4 border-b border-slate-800 bg-slate-800/50">
          <h3 class="font-bold text-lg text-white">Delete Task</h3>
        </div>
        <div class="p-6">
          <p class="text-slate-300 text-sm">Are you sure you want to delete <span class="font-semibold text-red-400">{{ deleteTargetTask?.title }}</span>? This action cannot be undone.</p>
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
import { useRoute } from 'vue-router'
import { useTaskStore } from '~/stores/taskStore'

const store = useTaskStore()
const route = useRoute()

const viewMode = ref(route.query.view === 'board' ? 'board' : 'list')
const search = ref('')
const filterStatus = ref('')
const filterPriority = ref('')
const filterProject = ref(route.query.project || '')
const filterStartDate = ref('')
const filterEndDate = ref('')
const selectedTaskIds = ref([])
const showCreateModal = ref(false)
const showTimeModal = ref(false)
const showDeleteModal = ref(false)
const deleteTargetTask = ref(null)
const editingTask = ref(null)
const selectedTask = ref(null)
const detailTask = ref(null)

// Board-specific drag state
const dragOverColumn = ref(null)
const draggingTaskId = ref(null)

const defaultForm = { title: '', description: '', category: 'General', priority: 'normal', project_id: 'default', status: 'pending', due_date: '', start_date: '' }
const form = ref({ ...defaultForm })
const timeForm = ref({ duration_minutes: 30, notes: '' })

onMounted(() => {
  store.fetchTasks()
  store.fetchProjects()
  store.fetchActiveTimer()
})

const priorityIcons = { urgent: '🔥', high: '⬆️', normal: '➡️', low: '⬇️' }
function priorityIcon(p) { return priorityIcons[p] || '➡️' }

function subtaskPercent(task) {
  if (!task.subtask_count) return 0
  return ((task.subtask_done || 0) / task.subtask_count) * 100
}

// Shared filtered tasks (used by both views)
const filteredTasks = computed(() => {
  let tasks = store.tasks
  
  if (search.value) {
    const q = search.value.toLowerCase()
    tasks = tasks.filter(
      (t) => t.title.toLowerCase().includes(q) || (t.description || '').toLowerCase().includes(q)
    )
  }
  
  if (filterStatus.value) {
    tasks = tasks.filter((t) => t.status === filterStatus.value)
  }

  if (filterPriority.value) {
    tasks = tasks.filter((t) => (t.priority || 'normal') === filterPriority.value)
  }

  if (filterProject.value) {
    tasks = tasks.filter((t) => t.project_id === filterProject.value)
  }
  
  if (filterStartDate.value) {
    tasks = tasks.filter((t) => {
       const d = t.start_date || t.created_at.split('T')[0]
       return d >= filterStartDate.value
    })
  }

  if (filterEndDate.value) {
     tasks = tasks.filter((t) => {
        const d = t.start_date || t.created_at.split('T')[0]
        return d <= filterEndDate.value
     })
  }
  
  return tasks
})

// Board-specific computed: split filteredTasks by status
const boardPending = computed(() => filteredTasks.value.filter(t => t.status === 'pending'))
const boardInProgress = computed(() => filteredTasks.value.filter(t => t.status === 'in_progress'))
const boardCompleted = computed(() => filteredTasks.value.filter(t => t.status === 'completed'))

const isAllSelected = computed(() => {
  return filteredTasks.value.length > 0 && selectedTaskIds.value.length === filteredTasks.value.length
})

function toggleSelectAll() {
  if (isAllSelected.value) {
    selectedTaskIds.value = []
  } else {
    selectedTaskIds.value = filteredTasks.value.map(t => t.id)
  }
}

async function handleBulkDelete() {
  if (!confirm(`Delete ${selectedTaskIds.value.length} tasks?`)) return
  await store.bulkDeleteTasks(selectedTaskIds.value)
  selectedTaskIds.value = []
}

function statusLabel(s) {
  const map = { pending: 'Pending', in_progress: 'In Progress', completed: 'Done' }
  return map[s] || s
}

function formatMin(m) {
  if (!m) return '0m'
  const h = Math.floor(m / 60)
  const mins = m % 60
  if (h === 0) return `${mins}m`
  if (mins === 0) return `${h}h`
  return `${h}h ${mins}m`
}

function formatElapsed(totalSec) {
  const h = Math.floor(totalSec / 3600)
  const m = Math.floor((totalSec % 3600) / 60)
  const s = totalSec % 60
  const pad = (n) => String(n).padStart(2, '0')
  if (h > 0) return `${pad(h)}:${pad(m)}:${pad(s)}`
  return `${pad(m)}:${pad(s)}`
}

function openDetail(task) { detailTask.value = task }

function openEdit(task) {
  detailTask.value = null
  editingTask.value = task
  form.value = {
    title: task.title,
    description: task.description,
    category: task.category,
    priority: task.priority || 'normal',
    project_id: task.project_id || 'default',
    status: task.status,
    due_date: task.due_date ? task.due_date.split('T')[0] : '',
    start_date: task.start_date ? task.start_date.split('T')[0] : '',
  }
}

function openEditFromDrawer(task) {
  detailTask.value = null
  openEdit(task)
}

function openTimeEntry(task) {
  selectedTask.value = task
  timeForm.value = { duration_minutes: 30, notes: '' }
  showTimeModal.value = true
}

function closeModals() {
  showCreateModal.value = false
  editingTask.value = null
  form.value = { ...defaultForm }
}

async function handleSave() {
  if (!form.value.title.trim()) return
  if (editingTask.value) {
    await store.updateTask(editingTask.value.id, form.value)
  } else {
    await store.createTask(form.value)
  }
  closeModals()
}

function handleDelete(task) {
  deleteTargetTask.value = task
  showDeleteModal.value = true
}

async function confirmDelete() {
  if (!deleteTargetTask.value) return
  await store.deleteTask(deleteTargetTask.value.id)
  showDeleteModal.value = false
  deleteTargetTask.value = null
}

async function handleTimeEntry() {
  if (!timeForm.value.duration_minutes || timeForm.value.duration_minutes < 1) return
  const now = new Date()
  const start = new Date(now.getTime() - timeForm.value.duration_minutes * 60000)
  await store.addTimeEntry(selectedTask.value.id, {
    start_time: start.toISOString(),
    end_time: now.toISOString(),
    duration_minutes: timeForm.value.duration_minutes,
    notes: timeForm.value.notes || '',
  })
  showTimeModal.value = false
  timeForm.value = { duration_minutes: 30, notes: '' }
}

async function handleStartTimer(task) {
  await store.startTimer(task.id)
}

async function handleStopTimer() {
  await store.stopTimer()
}

function formatDate(dateStr) {
  if (!dateStr) return ''
  return new Date(dateStr).toLocaleDateString('en-US', {
    day: 'numeric',
    month: 'short',
    year: 'numeric'
  })
}

// ===== Board Drag & Drop =====
function handleDragStart(event, task) { draggingTaskId.value = task.id; event.dataTransfer.effectAllowed = 'move' }
function handleDragEnd() { draggingTaskId.value = null; dragOverColumn.value = null }
function handleDragOver(col) { dragOverColumn.value = col }
function handleDragLeave() { dragOverColumn.value = null }

async function handleDrop(newStatus) {
  dragOverColumn.value = null
  const taskId = draggingTaskId.value
  if (!taskId) return
  const task = store.tasks.find(t => t.id === taskId)
  if (!task || task.status === newStatus) return
  try { await store.updateTask(taskId, { status: newStatus }) } catch (e) { console.error('Failed to update task status', e) }
}
</script>
