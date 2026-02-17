<template>
  <div class="flex h-screen bg-slate-950 text-slate-200 font-sans">
    <!-- Sidebar -->
    <aside class="w-64 bg-slate-900 border-r border-slate-800 flex flex-col shadow-sm">
      <div class="h-16 flex items-center px-6 border-b border-slate-800">
        <div class="flex items-center gap-3">
          <div>
            <h1 class="font-bold text-lg leading-tight uppercase tracking-wider text-white">Sic Mundus</h1>
            <div class="text-xs text-slate-500 font-medium">Creatus Est</div>
          </div>
        </div>
      </div>
      <nav class="flex-1 p-4 space-y-1">
        <NuxtLink to="/" class="flex items-center px-4 py-3 rounded-lg text-slate-400 hover:bg-slate-800 hover:text-white transition-colors" active-class="bg-slate-800 text-white font-medium">
          <span>Dashboard</span>
        </NuxtLink>
        <NuxtLink to="/projects" class="flex items-center px-4 py-3 rounded-lg text-slate-400 hover:bg-slate-800 hover:text-white transition-colors" active-class="bg-slate-800 text-white font-medium">
          <span>Projects</span>
        </NuxtLink>
        <NuxtLink to="/tasks" class="flex items-center px-4 py-3 rounded-lg text-slate-400 hover:bg-slate-800 hover:text-white transition-colors" active-class="bg-slate-800 text-white font-medium">
          <span>Tasks</span>
        </NuxtLink>

        <NuxtLink to="/time-log" class="flex items-center px-4 py-3 rounded-lg text-slate-400 hover:bg-slate-800 hover:text-white transition-colors" active-class="bg-slate-800 text-white font-medium">
          <span>Time Log</span>
        </NuxtLink>
        <NuxtLink to="/reports" class="flex items-center px-4 py-3 rounded-lg text-slate-400 hover:bg-slate-800 hover:text-white transition-colors" active-class="bg-slate-800 text-white font-medium">
          <span>Reports</span>
        </NuxtLink>
      </nav>
      <div class="p-4 border-t border-slate-800 text-center text-xs text-slate-600">
        <div>Sic Mundus v1.0</div>
        <div class="mt-1 opacity-50">made with ❤️ rifai</div>
      </div>
    </aside>

    <!-- Main -->
    <main class="flex-1 overflow-y-auto p-8 bg-slate-950">
      <div class="w-10/12 mx-auto">
        <slot />
      </div>
    </main>
  </div>
</template>
