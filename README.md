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
- **Authentication & Security**: Secure Login/Register with Argon2 hashing and JWT sessions.
- **Role Management**: Admin & User roles. Admins can manage users and reset passwords.
- **Premium Design**: Dark mode interface with glassmorphism, hover effects, and smooth transitions.

## 🛡️ Role System (3-Tier)

**Sic Mundus** now features a robust 3-tier role system:

| Role           | Access Level    | Capabilities                                                                                                        |
| -------------- | --------------- | ------------------------------------------------------------------------------------------------------------------- |
| **Superadmin** | 🟡 Gold Badge   | Full access. Can manage all users (including Admins), view all tasks, and see full time reports. Cannot be deleted. |
| **Admin**      | 🟣 Purple Badge | Department level access. Can manage **Users** (but not other Admins). Sees only User tasks and time reports.        |
| **User**       | ⬜ Slate Badge  | Personal access. Can only manage their own tasks and profile.                                                       |

> **Note:** The **first registered user** is automatically assigned the **Superadmin** role.

## 💾 Data Persistence & Deployment

This project uses a **Docker external volume** (`postgres_data`) to ensure database data is preserved across container rebuilds and deployments.

### Initial Setup (Production)

Run this command once on your server to create the persistent volume:

```bash
docker volume create postgres_data
```

### Deployment Scripts

We provide scripts to ensure safe deployments:

- **`scripts/backup-db.sh`**: Creates a timestamped backup of the database (schema + data) to `backups/`.
- **`scripts/deploy.sh`**: Automates the safe deployment process:
  1. Runs a database backup.
  2. Rebuilds and restarts containers (`docker compose up -d --build`).
  3. **Preserves data** (does NOT use `-v` flag).

**To deploy:**

```bash
./scripts/deploy.sh
```

## 📊 Enhanced Time Report (Admin)

The **Time Report** (`/admin/time-report`) provides deep insights into team productivity:

- **Monthly Filter**: Navigate between months to see historical performance.
- **Dual Totals**: View both **monthly** hours and **all-time** cumulative hours side-by-side.
- **Per-User Breakdown**: Detailed project-wise breakdown for each user with a clear **Total** row.
- **Summary Cards**: Quick stats for active users, total hours, and top performers.

## 🛠 Technology Stack

- **Frontend**: Nuxt 3, Vue 3, Pinia, Chart.js, Tailwind CSS
- **Backend**: Rust (Actix Web), PostgreSQL, SQLx, Argon2, JWT
- **Infrastructure**: Docker, Docker Compose, Shell Scripts

## 📦 How to Run

### Using Docker (Recommended)

1. Create volume (first time only):
   ```bash
   docker volume create postgres_data
   ```
2. Start services:
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
│       ├── handlers/         # API handlers (auth, user, task, entry, project, etc.)
│       ├── models/           # Data structures
│       └── routes.rs         # Route configuration
│
├── frontend-nuxt/            # Nuxt 3 Application
│   ├── layouts/              # App layout with sidebar navigation
│   ├── pages/                # Page components
│   │   ├── admin/            # Admin pages (User Management, Time Report)
│   │   ├── index.vue         # Dashboard
│   │   ├── tasks.vue         # Unified Tasks
│   │   └── ...
│   ├── stores/               # Pinia properties
│   └── middleware/           # Auth middleware
│
├── scripts/                  # Deployment & Maintenance scripts
│   ├── deploy.sh             # Safe deployment script
│   └── backup-db.sh          # Database backup script
│
├── docker-compose.yml        # Docker orchestration
└── README.md
```

## 📝 API Endpoints

### Authentication

| Method | Endpoint             | Description           |
| ------ | -------------------- | --------------------- |
| `POST` | `/api/auth/register` | Register new user     |
| `POST` | `/api/auth/login`    | Login and get JWT     |
| `GET`  | `/api/auth/me`       | Get current user info |

### Admin & Reports

| Method   | Endpoint                 | Description                                         |
| -------- | ------------------------ | --------------------------------------------------- |
| `GET`    | `/api/admin/time-report` | Get detailed time report (params: `?month=YYYY-MM`) |
| `GET`    | `/api/users`             | List users (Role-aware)                             |
| `DELETE` | `/api/users/{id}`        | Delete user (Role-aware)                            |
| `PUT`    | `/api/users/{id}/role`   | Change user role                                    |

---

_Made with ❤️_
