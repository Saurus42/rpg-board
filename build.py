import sys
import subprocess
import shutil
if shutil.which( "cargo" ) == None:
    print( "Please install cargo." )
    exit( 0 )

builder = {
    "linux": "x86_64-unknown-linux-gnu",
    "windows": "x86_64-pc-windows-msvc",
    "rpi-linux": "aarch64-unknown-linux-gnu"
}

try:
    platform = ""
    target = ""
    for plat in sys.argv:
        data = plat.split( "=" )
        if data[0] == "platform":
            platform = "--target=" + builder[data[1]]
        if data[0] == "target":
            target = "--" + data[1]
    print( "Application is building. Please await." )
    args = f"{platform} {target}"
    subprocess.check_output( f"cargo build {args}" )
except subprocess.CalledProcessError as error:
    print( error.output )