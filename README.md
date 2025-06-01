<style>h1 { border-bottom: 0; } </style>

<p align="center">
    <img width="150" src="./public/logo.png" alt="logo">
</p>
<h1 align="center">Game Power Hammer</h1>

## Technologies run-down

- Nuxt 3 (v4 ready!)
- Tauri 2
- NuxtUI v3
- TailwindCSS v4
- Typescript
- ESLint
- Auto imports (for Tauri api too!)

## Functionalities

- Send custom notifications to the client (remember to turn on/grant notifications in your computer settings)
- Store and retrieve data locally
- Show tray icon
- Support all Nuxt functionalities (routing/layout/middleware/modules/etc...)

## Setup

  - Follow these commands:

  ```sh
  # install dependencies
  $ pnpm install

  # start the project
  $ pnpm run tauri:dev
  ```

## Build

  ```sh
  $ pnpm run tauri:build
  ```

This command will generate the Nuxt static output and bundle the project under `src-tauri/target`.

## Debug

  ```sh
  $ pnpm run tauri:build:debug
  ```

The same Tauri bundle will generate under `src-tauri/target`, but with the ability to open the console.

## Thanks

Many thanks to [NicolaSpadari](https://github.com/NicolaSpadari) for the base Nuxt and Tauri integration.

## License

MIT License © 2024-PRESENT [VALERE91](https://github.com/VALERE91)
