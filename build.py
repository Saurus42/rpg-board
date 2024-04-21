# Importing necessary modules for system interaction and subprocess execution.
import sys
import subprocess
import shutil

# Checking if Cargo is installed by attempting to locate it in the system PATH.
if shutil.which("cargo") is None:
    print("Please install cargo.")
    exit(0)

# Dictionary mapping different platforms to their corresponding builder targets in Rust.
builder = {
    "linux": "x86_64-unknown-linux-gnu",
    "windows": "x86_64-pc-windows-msvc",
    "rpi-linux": "aarch64-unknown-linux-gnu"
}

try:
    platform = ""
    target = ""
    # Parsing command line arguments to determine the platform and target.
    for argv in sys.argv:
        data = argv.split("=")
        if data[0] == "platform":
            platform = "--target=" + builder[data[1]]
        if data[0] == "target":
            target = "--" + data[1]
    # Informing the user that the application is being built.
    print("Application is building. Please await.")
    # Constructing the arguments for Cargo build command.
    args = f"{platform} {target}"
    # Initiating the build process using Cargo.
    subprocess.check_output(f"cargo build {args}")
except subprocess.CalledProcessError as error:
    # Handling errors by printing the output, if any, to aid in troubleshooting.
    print(error.output)
