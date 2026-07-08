# MDView

A lightweight, free markdown viewer for Windows. Double-click any `.md` file and see it beautifully formatted — GitHub style, with syntax-highlighted code blocks, a table of contents, dark/light themes, and print-to-PDF.

Built with [Tauri](https://tauri.app), so the installer is only a few MB (it uses the WebView2 engine already built into Windows 10/11).

## Features

- Opens `.md`, `.markdown`, `.mdown`, `.mkd` files via double-click ("Open with") after install
- Drag & drop any markdown file onto the window
- GitHub-flavored markdown: tables, task lists, strikethrough, fenced code
- Syntax highlighting for ~40 common languages
- Dark / light theme toggle (remembers your choice)
- Table of contents sidebar generated from headings
- Print / Save as PDF with a clean print layout
- Reload button (F5) to re-read the file after edits
- Fully offline — no network access, no telemetry

## How to get the installer (no tools needed on your PC)

The Windows installer is built automatically by GitHub Actions:

1. Create a **free GitHub account** and a new **public repository** (e.g. `mdview`).
2. Upload this whole folder to the repository (drag the contents into GitHub's "Add file → Upload files" page, or use `git push`). Make sure `.github/workflows/build.yml` is included.
3. Go to the repo's **Actions** tab. The "Build Windows Installer" workflow starts automatically (or press **Run workflow**).
4. Wait ~10 minutes. Open the finished run and download the **MDView-Windows-Installer** artifact — it contains `MDView_1.0.0_x64-setup.exe`.

### Publishing for everyone

To give the app away, create a tag — GitHub then publishes the installer as a public Release anyone can download:

```
git tag v1.0.0
git push origin v1.0.0
```

Share the link: `https://github.com/<your-username>/mdview/releases/latest`

> **Note on SmartScreen:** the installer is unsigned (code-signing certificates cost money), so Windows may show a "Windows protected your PC" prompt. Users click **More info → Run anyway**. This is normal for free open-source apps.

## Building locally (optional)

Requires [Rust](https://rustup.rs) and [Node.js](https://nodejs.org):

```
npm install -D @tauri-apps/cli
npx tauri build        # installer appears in src-tauri/target/release/bundle/nsis/
npx tauri dev          # run in development mode
```

## Project layout

```
ui/                  Frontend (single index.html + vendored libraries, fully offline)
src-tauri/           Rust backend, app config, icons, file-association setup
.github/workflows/   CI that builds the Windows installer
```

## License

MIT — free for everyone.
