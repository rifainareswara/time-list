# Sic Mundus · Time Tracker

**Sic Mundus** is a modern, fast, and aesthetic task management and time tracking application. Built with love for productivity.

## 🚀 Key Features

- **Task Management**: Create, edit, and organize tasks with priorities (Low / Medium / High / Urgent), statuses, and subtasks.
- **Unified Tasks View**: Switch between **List** view (table) and **Board** view (Kanban columns) with a single toggle.
- **Project Management**: Organize tasks by project with color coding, statistics, and search.
- **Time Tracking**: Built-in timer with Start/Stop and manual time logging.
- **Time Log**: Full work history with duration, notes, and date filtering.
- **Visual Dashboard**: Daily activity bar chart, **Time per Project** donut chart, project details breakdown, and recent notes.
- **Reports**: Analytics and productivity reports.
- **Premium Design**: Dark mode interface with glassmorphism, hover effects, and smooth transitions.

## 🛠 Technology Stack

- **Frontend**: Nuxt 3, Vue 3, Pinia, Chart.js, Tailwind CSS
- **Backend**: Rust (Actix Web), PostgreSQL, SQLx
- **Containerization**: Docker & Docker Compose

## 📦 How to Run

### Using Docker (Recommended)

```bash
docker compose up -d --build
```

The application will be available at:

- **Frontend**: http://localhost:8005
- **Backend API**: http://localhost:8006

### Manual Setup (Development)

Prerequisites: Rust (cargo), Node.js (npm), PostgreSQL.

1. **Run Backend**:

   ```bash
   cd backend
   cargo run
   ```

   Backend runs on port `8006`.

2. **Run Frontend**:
   ```bash
   cd frontend-nuxt
   npm install
   npm run dev
   ```
   Frontend runs on port `8005`.

## 📂 Project Structure

```
├── backend/                  # Rust Actix Web API
│   └── src/
│       ├── db/               # PostgreSQL connection & migrations
│       ├── handlers/         # API handlers (task, entry, project, subtask, timer, dashboard)
│       ├── models/           # Data structures
│       └── routes.rs         # Route configuration
│
├── frontend-nuxt/            # Nuxt 3 Application
│   ├── layouts/              # App layout with sidebar navigation
│   ├── pages/                # Page components
│   │   ├── index.vue         # Dashboard (charts, stats, recent activity)
│   │   ├── tasks.vue         # Unified Tasks (List + Board view toggle)
│   │   ├── projects.vue      # Project management with search
│   │   ├── time-log.vue      # Time log history
│   │   └── reports.vue       # Reports & analytics
│   ├── stores/               # Pinia state management
│   └── nuxt.config.ts        # Nuxt config (SPA mode, proxy)
│
├── docker-compose.yml        # Docker orchestration (frontend, backend, PostgreSQL)
└── README.md
```

## 📝 API Endpoints

### Tasks

| Method   | Endpoint                 | Description       |
| -------- | ------------------------ | ----------------- |
| `GET`    | `/api/tasks`             | Fetch all tasks   |
| `POST`   | `/api/tasks`             | Create a new task |
| `PUT`    | `/api/tasks/{id}`        | Update a task     |
| `DELETE` | `/api/tasks/{id}`        | Delete a task     |
| `POST`   | `/api/tasks/bulk-delete` | Bulk delete tasks |

### Subtasks

| Method   | Endpoint                   | Description             |
| -------- | -------------------------- | ----------------------- |
| `GET`    | `/api/tasks/{id}/subtasks` | Get subtasks for a task |
| `POST`   | `/api/tasks/{id}/subtasks` | Create a subtask        |
| `PUT`    | `/api/subtasks/{id}`       | Update a subtask        |
| `DELETE` | `/api/subtasks/{id}`       | Delete a subtask        |

### Time Entries

| Method   | Endpoint                  | Description            |
| -------- | ------------------------- | ---------------------- |
| `GET`    | `/api/entries`            | Get all time entries   |
| `GET`    | `/api/tasks/{id}/entries` | Get entries for a task |
| `POST`   | `/api/tasks/{id}/entries` | Create a time entry    |
| `DELETE` | `/api/entries/{id}`       | Delete a time entry    |

### Projects

| Method   | Endpoint             | Description                     |
| -------- | -------------------- | ------------------------------- |
| `GET`    | `/api/projects`      | Fetch all projects (with stats) |
| `POST`   | `/api/projects`      | Create a project                |
| `PUT`    | `/api/projects/{id}` | Update a project                |
| `DELETE` | `/api/projects/{id}` | Delete a project                |

### Timer

| Method | Endpoint                     | Description            |
| ------ | ---------------------------- | ---------------------- |
| `POST` | `/api/timer/start/{task_id}` | Start timer for a task |
| `POST` | `/api/timer/stop`            | Stop active timer      |
| `GET`  | `/api/timer/active`          | Get active timer info  |

### Dashboard

| Method | Endpoint                 | Description                                 |
| ------ | ------------------------ | ------------------------------------------- |
| `GET`  | `/api/dashboard/summary` | Dashboard stats, charts & project breakdown |

---

_Made with ❤️ by rifai_
