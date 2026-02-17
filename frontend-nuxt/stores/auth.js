import { defineStore } from 'pinia'

export const useAuthStore = defineStore('auth', {
  state: () => ({
    user: null,
    token: useCookie('auth_token').value || null,
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
        return true
      } catch (err) {
        console.error('Login failed', err)
        return false
      }
    },

    async register(username, password) {
      try {
        const { data, error } = await useFetch('/api/auth/register', {
          method: 'POST',
          body: { username, password },
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
        const { data, error } = await useFetch('/api/auth/me', {
          headers: { Authorization: `Bearer ${this.token}` },
        })

        if (error.value) {
          this.logout()
          return
        }

        this.user = { 
          id: data.value.sub, 
          role: data.value.role,
          // We don't get username from 'me' endpoint currently, but that's fine for now
          // or we could update 'me' to return full user object.
          // For now, let's just stick with what we have.
        }
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
