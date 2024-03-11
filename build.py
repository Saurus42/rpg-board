import sys
import subprocess
import shutil
if shutil.which( "cargo" ) == None:
    print( "Please install cargo." )
    exit( 0 )

try:
    print( "Build application. Please await." )
    args = ""
    for index in range( 1, len( sys.argv ) ):
        args += sys.argv[index] + " "
    subprocess.check_output( f"cargo build {args}" )
except subprocess.CalledProcessError as error:
    print( error.output )