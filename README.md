# Spacepix 🌌

[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL--3.0-blue.svg)](https://opensource.org/licenses/AGPL-3.0)

A desktop application for discovering exciting facts and stunning images about space. Made with **Rust**, **Tauri**, and **Leptos**, powered by the **NASA API**!

## Table of Contents

*   [About Spacepix](#about-spacepix)
*   [Features](#features)
*   [Technologies Used](#technologies-used)
*   [Getting Started](#getting-started)
    *   [Prerequisites](#prerequisites)
    *   [Setup Instructions](#setup-instructions)
*   [Contributing](#contributing)
    *   [Contribution Guidelines](#contribution-guidelines)
    *   [Branching and Naming](#branching-and-naming)
    *   [Code Reviews](#code-reviews)
    *   [Testing](#testing)
    *   [Pull Request Templates](#pull-request-templates)
    *   [Pre-Commit Hooks](#pre-commit-hooks)
*   [Roadmap](#roadmap)
*   [License](#license)
*   [Community](#community)

## About Spacepix

Spacepix is an open-source desktop application designed for space enthusiasts and curious minds. It leverages the power of the NASA Astronomy Picture of the Day (APOD) API to bring a daily dose of cosmic wonder directly to your desktop. Discover captivating facts and breathtaking images of our universe, all within a beautiful and performant application.

## Features

*   **Daily APOD:** View the NASA Astronomy Picture of the Day with detailed explanations.
*   **Image Discovery:** Explore a rich archive of space images.
*   **Cross-Platform:** Available on Windows, macOS, and Linux thanks to Tauri.
*   **Fast & Efficient:** Built with Rust for native performance.

## Technologies Used

*   **Rust:** The core language for the application logic and backend.
*   **Tauri:** A framework for building cross-platform desktop applications with web frontends.
*   **Leptos:** A full-stack, declarative UI framework for building web applications with Rust.
*   **NASA APOD API:** Source for all astronomical images and data.

## Getting Started

Follow these instructions to get a copy of Spacepix up and running on your local machine for development and testing purposes.

### Prerequisites

Before you begin, ensure you have the following installed:

*   **Rust and Cargo:** Follow the official Rust installation guide: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
*   **Node.js & npm/yarn:** Tauri uses Node.js for its build process.
    [https://nodejs.org/en/download](https://nodejs.org/en/download)
*   **Tauri Prerequisites:** Depending on your operating system, you might need additional dependencies. Refer to the official Tauri documentation for your OS: [https://tauri.app/v1/guides/getting-started/prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)
    *   **For Debian/Ubuntu (Linux):**
        ```bash
        sudo apt update
        sudo apt install libwebkit2gtk-4.0-dev libayatana-appindicator3-dev librsvg2-dev build-essential curl wget file
        ```

### Setup Instructions

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/CodeCanna/spacepix.git
    cd spacepix
    ```

2.  **Install frontend dependencies (if any):**
    Leptos applications often have minimal `npm` dependencies, but it's good practice to run this if a `package.json` exists.
    ```bash
    npm install # or yarn install
    ```

3.  **Run in development mode:**
    This will compile the Rust code and launch the Tauri application.
    ```bash
    cargo tauri dev
    ```

4.  **Build for release:**
    To create an optimized, standalone executable:
    ```bash
    cargo tauri build
    ```
    The executables will be found in the `src-tauri/target/release` directory (or platform-specific subdirectories).

## Contributing

We welcome contributions from everyone! Whether you're a seasoned Rustacean, a web developer, a designer, or a space enthusiast, your help is invaluable.

Please read our contribution guidelines carefully before submitting your first pull request.

### Contribution Guidelines

*   **Open an Issue:** Before starting any significant work, please open an issue to discuss your proposed changes. This helps avoid duplicate efforts and ensures your contribution aligns with the project's direction.
*   **Respectful Communication:** Be kind and respectful to other contributors.

### Branching and Naming

*   **Always open a new branch for each issue.**
*   **Branch Naming Convention:**
    *   For bug fixes: `bugs/{bug_explanation}` (e.g., `bugs/fix-image-load-error`)
    *   For new features: `feat/{feat_explanation}` (e.g., `feat/add-search-apod`)

### Code Reviews

*   **Every single piece of code must be peer-reviewed** before being merged into the `master`/`main` branch.
*   When you, as the responsible developer, feel your Pull Request (PR) is ready for review, **mark the associated Issue** (e.g., as `needs-review`).

### Testing

*   **Required Testing Coverage:** Ensure that **testing coverage is at least 80%** before tagging reviewers for a Pull Request.

### Pull Request Templates

*   We utilize **PR templates** to guide contributors. These templates include checklists to help you remember important tasks and provide a clear structure for your submission. This is especially helpful for new contributors.

### Pre-Commit Hooks

*   We encourage implementing a `.pre-commit` file to enforce consistent code formatting (e.g., End-Of-File (EOF) newlines, trailing whitespaces, tab usage). This helps maintain code quality and reduces merge conflicts.

## Roadmap

*   Implement APOD search by date.
*   Add a favorites section for saved images.
*   Improve UI/UX with more advanced Leptos components.
*   Introduce CI/CD via GitHub Actions for automated testing and builds.

## License

This project is licensed under the **AGPL-3.0 License** - see the [LICENSE](LICENSE) file for details.

## Community

Join our Discord community to discuss Spacepix, get support, and connect with other contributors!

[**Join the Spacepix Discord!**](https://discord.gg/your-discord-invite-link-here)
*(Replace `your-discord-invite-link-here` with your actual Discord invite link)*