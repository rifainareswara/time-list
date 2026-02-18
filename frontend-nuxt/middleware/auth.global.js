import { useAuthStore } from '~/stores/auth'

export default defineNuxtRouteMiddleware((to, from) => {
  const auth = useAuthStore()

  // Public routes
  if (to.path === '/login' || to.path === '/register') {
    if (auth.isAuthenticated) {
      return navigateTo('/')
    }
    return
  }

  // Protected routes
  if (!auth.isAuthenticated) {
    return navigateTo('/login')
  }

  // Force Change Password
  if (auth.user?.force_change_password && to.path !== '/change-password') {
    return navigateTo('/change-password')
  }

  // Prevent accessing change-password if not required (optional, but good UX)
  if (!auth.user?.force_change_password && to.path === '/change-password') {
     return navigateTo('/')
  }

  // Admin routes
  if (to.path.startsWith('/admin') && !auth.isAdmin) {
    // If we haven't fetched user yet but have token, we might not know if admin.
    // However, the plugin should handle fetching user.
    // If still not admin, redirect.
    return navigateTo('/')
  }
})
