# 🧭 CloudMasterApp — Nuxt 4 + Tauri Desktop App

A modern cross-platform desktop application built with Nuxt 4 (Vue 3 + Vite) and Tauri (Rust). This repo provides a lightweight desktop shell for a Nuxt web app.

## Highlights

- Nuxt 4 + Vue 3
- Vite for fast HMR
- Tauri for a secure, small desktop binary
- Yarn (Classic) as package manager

## Prerequisites

- Node.js >= 22
- Yarn 1.22.x (the project uses Yarn Classic)
- Rust toolchain (for Tauri builds): install via https://rustup.rs
- Platform-specific Tauri requirements (see https://tauri.app/v1/guides/getting-started/prerequisites)

## Quick Start

1. Clone and install

   ```bash
   git clone https://github.com/netherguy4/CloudMasterApp.git
   cd CloudMasterApp
   yarn install
   ```

2. Run the web app (Nuxt dev)

   ```bash
   yarn dev
   # visit http://localhost:3000 (or as reported by Nuxt)
   ```

3. Run the desktop app (Tauri)
   - In most workflows you can run the Tauri dev command which runs the backend and launches a desktop window:

     ```bash
     yarn tauri:dev
     ```

   - If you want to run the Nuxt dev server separately and then start Tauri, open two terminals:

     ```bash
     # Terminal A
     yarn dev

     # Terminal B
     yarn tauri:dev
     ```

## Build & Package

- Generate static site (if used)

  ```bash
  yarn build
  ```

- Build Tauri production binary

  ```bash
  yarn tauri:build
  ```

  This produces platform-specific installers/binaries under the Tauri target directory.

## Useful Scripts

- `yarn dev` — Nuxt dev server (with --host)
- `yarn build` — Nuxt generate
- `yarn tauri:dev` — Start Tauri in development
- `yarn tauri:build` — Build production desktop binaries
- `yarn lint` — Run ESLint across the repo
- `yarn postinstall` — Nuxt prepare (runs automatically after install)

## Project layout (high-level)

- `/app` — Nuxt application source (pages, components, layouts)
- `/src-tauri` — Tauri configuration and Rust backend

## Contributing

- Follow existing ESLint & Prettier rules.
- Run linters before opening a PR:

  ```bash
  yarn lint
  ```

## Troubleshooting

- If Tauri build fails due to missing Rust toolchain, install Rust via rustup and ensure cargo is on PATH.
- For Node/Yarn version issues, use a version manager (nvm, corepack, or similar) to align with prerequisites.

<!-- End of README -->
