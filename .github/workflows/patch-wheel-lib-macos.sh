#!/bin/bash

# Check if the correct number of arguments are provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <wheel_dir>"
    echo "The script takes a wheel directory and for each wheel found named *.whl "
    echo "it patches it by searching for a lib that starts with '_extractous*.so' "
    echo "then it changes the name of libtika_native.dylib to be @loader_path/libs/libtika_native.dylib'"
    exit 1
fi

WHEEL_DIR=$1

# Check if the provided wheel directory exists
if [ ! -d "$WHEEL_DIR" ]; then
    echo "Wheel directory does not exist: $WHEEL_DIR"
    exit 1
fi

# Ensure wheel and patchelf are installed
if ! command -v install_name_tool &> /dev/null
then
    echo "install_name_tool could not be found"
    exit 1
fi

if ! command -v otool &> /dev/null
then
    echo "otool could not be found"
    exit 1
fi

# Find all wheel files in the specified directory
WHEEL_FILES=$(find "$WHEEL_DIR" -name "*.whl")

# Check if any wheel files were found
if [ -z "$WHEEL_FILES" ]; then
    echo "No wheel files found in the directory: $WHEEL_DIR"
    exit 1
fi

# Loop through each wheel file and perform the required operations
for WHEEL_FILE in $WHEEL_FILES; do
    echo "Processing $WHEEL_FILE ..."

    # Unpack the wheel file into the wheel directory
    python -m wheel unpack "$WHEEL_FILE" -d "$WHEEL_DIR"

    # Find the directory containing the unpacked wheel contents
    UNPACKED_WHEEL_DIR=$(find "$WHEEL_DIR" -mindepth 1 -maxdepth 1 -type d -name "extractous*")

    # Find the .so file in the extractous directory
    SO_FILE=$(find "$UNPACKED_WHEEL_DIR" -name "_extractous*.so" | head -n 1)

    # Check if the .so file exists
    if [ -z "$SO_FILE" ]; then
        echo "No file starting with _extractous found in the extractous directory of $WHEEL_FILE"
        continue
    fi

    # Extract the library names using otool
    LIB_TIKA_NATIVE=$(otool -l "$SO_FILE" | grep libtika_native | awk '{print $2}')

    # Check if libtika is found
    if [ ! -f "$LIB_TIKA_NATIVE" ]; then
        echo "$LIB_TIKA_NATIVE not found in $SO_FILE"
        exit 1
    fi
    echo "patching library  $LIB_TIKA_NATIVE"

    # Change the library path using install_name_tool
    install_name_tool -change "$LIB_TIKA_NATIVE" "@loader_path/$(basename $LIB_TIKA_NATIVE)" "$SO_FILE"

    # Pack the wheel again
    python -m wheel pack "$UNPACKED_WHEEL_DIR" -d "$WHEEL_DIR"

    # Clean up the unpacked directory
    rm -rf "$UNPACKED_WHEEL_DIR"

    echo "Wheel file $WHEEL_FILE has been patched and repacked successfully."
done