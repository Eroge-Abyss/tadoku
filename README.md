# Tadoku - Visual Novel Tracker

<p align="center">
  <img src="assets/icon.png" alt="Tadoku App Icon" style="width: 150px; height: auto;">
</p>

Tadoku is a modern, blazing-fast Visual Novel (VN) tracker designed for Japanese learners and casual readers. It helps you manage your VN library, track playtime, and organize your reading progress. Built with **Tauri** and **SvelteKit**, Tadoku offers a lightweight, cross-platform experience with a sleek and intuitive user interface.

---

## Features

- **Track Playtime**: Automatically tracks how much time you've spent reading each Visual Novel.
- **Game Management**: Add, edit, and remove Visual Novels from your library.
- **NSFW Filter**: Blurs NSFW images by default
- **Pinned Games**: Pin your favorite games for quick access.
- **Character Tracking**: Optionally include character information for each Visual Novel.
- **Cross-Platform**: Works on Windows, with basic Linux support (need to build from source).
- **Discord Rich Presence**: Share your current reading activity on Discord.
<!-- not added yet - **Customizable**: Organize games with categories and tags. -->

---

## Screenshots

<!-- Add your screenshots here -->

### Main Library View
![Screenshot 1](assets/main_library.png)


### Game Details Page
![Screenshot 2](assets/game_details.png)


### Add Game Modal
![Screenshot 3](assets/add_game.png)


### Characters Page
![Screenshot 4](assets/characters.png)

---

## Installation

### Prerequisites

- **Rust**: Install Rust from [rustup.rs](https://rustup.rs/).
- **Bun**: Install Bun from [bun.sh](https://bun.sh/).

### Steps

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/tadoku.git
   cd tadoku
   ```

2. Install dependencies:

   ```bash
   bun install
   ```

3. Build and run the app:

   ```bash
   bun run tauri dev
   ```

4. For production builds:
   ```bash
   bun run tauri build
   ```

---

## Usage

1. **Add a Game**:

   - Click the `+` button to add a new Visual Novel.
   - Search for the game by name or ID using the VNDB API.
   - Select the game and provide the path to the executable.

2. **Track Playtime**:

   - Start a game from the library, and Tadoku will automatically track your playtime.
   - Playtime is displayed in hours and minutes on the game's details page.

3. **Pin Games**:

   - Pin your favorite games to the sidebar for quick access.

4. **View Characters**:

   - If enabled, view character information for each Visual Novel.

5. **Discord Integration**:
   - Tadoku updates your Discord status with the current game you're playing.

---

## Contributing

Contributions are welcome! If you'd like to contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Submit a pull request with a detailed description of your changes.

---

## Acknowledgments

- **Tauri**: For providing a lightweight framework to build cross-platform apps.
- **VNDB**: For their comprehensive Visual Novel database API.
- **SvelteKit**: For enabling a fast and reactive frontend.

---

## Support

If you encounter any issues or have suggestions, please open an issue on the [GitHub repository](https://github.com/Eroge-Abyss/tadoku/issues).

---

Enjoy tracking your Visual Novels with Tadoku! 📚✨
