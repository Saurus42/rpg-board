# Bot README

This bot is designed to handle commands related to rolling dice and providing help information about commands.

## Installation and Configuration

### Prerequisites

- Rust programming language installed (https://www.rust-lang.org/tools/install)
- Cargo package manager (automatically installed with Rust)
- Python installed (for using the build script)

### Installation Steps

1. Clone the repository to your local machine:

   ```bash
   git clone https://github.com/your_username/your_bot.git
   ```

2. Navigate to the project directory:

   ```bash
   cd your_bot
   ```

3. Create a `.env` file in the project root and add your bot token:

   ```plaintext
   token=YOUR_DISCORD_BOT_TOKEN
   ```

4. Use the provided build script (`build.py`) to build the project:

   ```bash
   python build.py platform=linux
   ```

   Replace `platform=linux` with the appropriate platform if needed (`windows`, `rpi-linux`, etc.).

   Alternatively, you can build the project using Cargo directly:

   ```bash
   cargo build --release
   ```

### Running the Bot

After successfully building the project, you can run the bot using the following command:

```bash
cargo run --release
```

### Inviting the Bot to Your Discord Server

To invite the bot to your Discord server, you need to create an OAuth2 URL with the "bot" scope and the necessary permissions.

1. Go to the Discord Developer Portal (https://discord.com/developers/applications) and select your bot application.
2. Navigate to the "OAuth2" section.
3. Under "OAuth2 URL Generator", select the "bot" scope.
4. Select the required permissions for your bot (e.g., "Send Messages", "Manage Messages", etc.).
5. Copy the generated URL and paste it into your browser. Follow the prompts to invite the bot to your server.

### Configuring Commands

Commands such as `/roll` and `/help` are automatically registered when the bot starts. You can customize or add new commands by modifying the respective Rust files (`roll.rs`, `help.rs`, etc.) and registering them in the `main.rs` file.

## Usage

Once the bot is running and added to your Discord server, you can use commands by typing them in a channel where the bot is present. For example:

- `/roll 2d6`: Rolls two six-sided dice.
- `/help roll`: Provides information about the `/roll` command.

Enjoy using your bot!