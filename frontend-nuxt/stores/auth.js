import { defineStore } from 'pinia'

export const useAuthStore = defineStore('auth', {
  state: () => ({
    user: null,
    token: useCookie('auth_token').value || null,
    forceChangePassword: false,
  }),
  getters: {
    isAuthenticated: (state) => !!state.token,
    isAdmin: (state) => state.user?.role === 'admin',
  },
  actions: {
    async login(username, password) {
      try {
        const { data, error } = await useFetch('/api/auth/login', {
          method: 'POST',
          body: { username, password },
        })

        if (error.value) throw error.value

        this.setToken(data.value.token)
        this.user = data.value.user
        this.forceChangePassword = data.value.user.force_change_password
        return true
      } catch (err) {
        console.error('Login failed', err)
        return false
      }
    },

    async changePassword(oldPassword, newPassword) {
      try {
        const { error } = await useFetch('/api/password', {
          method: 'PUT',
          headers: { Authorization: `Bearer ${this.token}` },
          body: { old_password: oldPassword, new_password: newPassword }
        })

        if (error.value) throw error.value
        
        this.forceChangePassword = false
        // Update user object locally
        if (this.user) {
           this.user.force_change_password = false
        }
        return true
      } catch (err) {
        console.error('Password change failed', err)
        return false
      }
    },

    async register(username, password, fullName = '') {
      try {
        const { data, error } = await useFetch('/api/auth/register', {
          method: 'POST',
          body: { username, password, full_name: fullName },
        })

        if (error.value) throw error.value

        this.setToken(data.value.token)
        this.user = data.value.user
        return true
      } catch (err) {
        console.error('Registration failed', err)
        return false
      }
    },

    async fetchUser() {
      if (!this.token) return

      try {
        const { data, error } = await useFetch('/api/me', {
          headers: { Authorization: `Bearer ${this.token}` },
        })

        if (error.value) {
          this.logout()
          return
        }

        this.user = data.value
        this.forceChangePassword = data.value.force_change_password
      } catch (err) {
        this.logout()
      }
    },

    logout() {
      this.token = null
      this.user = null
      const cookie = useCookie('auth_token')
      cookie.value = null
      navigateTo('/login')
    },

    setToken(token) {
      this.token = token
      const cookie = useCookie('auth_token', { maxAge: 60 * 60 * 24 * 7 }) // 7 days
      cookie.value = token
    },
  },
})
