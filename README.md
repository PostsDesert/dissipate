# Dissipate - Personal Microblogging Platform

A Twitter-like microblogging platform where users only see their own messages.

## Project Structure

```
dissipate/
├── backend/          # Rust/Axum backend
├── frontend/         # SolidJS frontend
├── database/         # SQLite database files
└── PLAN.md           # Implementation plan
```

## Development

You can start both the backend and frontend simultaneously using the provided wrapper script in the root directory:

```bash
./dev.sh
```

### Backend Only

```bash
cd backend
cargo run
```

### Frontend Only

```bash
cd frontend
npm install
npm run dev
```


## Environment Variables

Copy `.env.example` to `.env` and configure:

**Backend (.env):**
- `DATABASE_URL` - SQLite database path
- `JWT_SECRET` - JWT signing secret
- `RUST_LOG` - Log level

**Frontend (.env):**
- `VITE_API_URL` - Backend API URL
