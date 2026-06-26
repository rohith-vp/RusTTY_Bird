# RusTTY Bird 🐦‍🔥

A lightweight, high-performance terminal-based Flappy Bird clone written in **Rust**.

![screenshot of RusTTY_Bird](RusTTY_Bird.png)

---

## 🏗️ Architecture & Libraries

`RusTTY Bird` uses a classic game loop architecture structured across individual module components:

* **`crossterm`**: The core terminal manipulation library used for handling raw input events (keyboard space/jump inputs), terminal resizing, cursor hiding, and cross-platform color rendering.
* **`rand`**: Used inside the pipe mechanics to dynamically generate variable pipe gap heights.

---

## 🚀 How to Run the Game

### Method 1: Download the Pre-compiled Binary (Quickest)

You do not need Rust installed on your system to play the game.

1. Navigate to the **Releases** section on the right side of this GitHub repository page.
2. Download the latest compiled executable binary compatible with your Operating System (e.g., Windows, macOS, or Linux).
3. Open your terminal or command prompt, navigate to your downloads folder, and execute the file:
```bash
# Linux/macOS
chmod +x rustty_bird
./rustty_bird

# Windows
.\rustty_bird.exe
```

### Method 2: Build and Run from Source

If you have the Rust toolchain installed, you can easily compile and run the project locally.

1. Clone the repository:

```bash
git clone [https://github.com/yourusername/RusTTY_Bird.git](https://github.com/yourusername/RusTTY_Bird.git)
cd RusTTY_Bird
```

2. Run the game using Cargo:
> Cargo will automatically fetch the dependencies and run the application in a unified step:

```bash
cargo run --release
```

---

## 🎮 Controls

- `Spacebar` or `Enter` — Jump
- `Q` or `Esc` — Quit Game
- `Ctrl + C` — Force Exit
