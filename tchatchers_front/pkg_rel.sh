rustup target list --installed

#!/bin/sh

# Function to print step information
print_step() {
    echo -e "\033[1;37m===== ${1} ${2} =====\033[0m"
}

# Function to print command information
print_command() {
    echo -e "\033[0;36mFollowing command will be executed: ${1}\033[0m"
}

# Function to print error messages in red and exit
print_error() {
    echo -e "\033[0;31mError: $1\033[0m"
    cleanup_and_exit 1
}

# Function to print user interruption message in orange and exit
print_user_interruption() {
    echo -e "\033[0;33mScript execution interrupted by user.\033[0m"
    cleanup_and_exit 24
}

# Function to perform cleanup and exit
cleanup_and_exit() {
    echo "Cleaning up..."
    # Kill all background jobs
    kill "${pids[@]}" &>/dev/null
    exit "$1"
}

# Function to print and execute a command
execute_command() {
    local cmd="$1"
    echo -e "\033[0;36mFollowing command will be executed: ${cmd}\033[0m"
    eval "$cmd"
}

# Function to get the current timestamp
get_timestamp() {
    date +"%Y-%m-%d %H:%M:%S"
}

# Trap Ctrl+C and Ctrl+D signals to perform cleanup before exiting
trap 'print_user_interruption' INT QUIT

# Record the start time
start_time=$(date +%s)

# Check if cargo exists
print_step "Step 0:" "Check if cargo exists"
if ! command -v cargo &> /dev/null; then
    print_error "cargo not found. Please install cargo before running this script."
fi

# Check if wasm32-unknown-unknown target is installed
print_step "Step 0:" "Check if wasm32-unknown-unknown target is installed"
if ! rustup target list --installed | grep -q 'wasm32-unknown-unknown'; then
    print_error "wasm32-unknown-unknown target not installed. Please run 'cargo install --force wasm-pack' to install it."
fi

# Step 1: Check if wasm-pack exists
print_step "Step 1:" "Check if wasm-pack exists"
if ! command -v wasm-pack &> /dev/null; then
    print_error "wasm-pack not found. Please install wasm-pack before running this script."
fi

# Step 2: Create the release folder if it doesn't exist
print_step "Step 2:" "Create the release folder if it doesn't exist"
folder="release"
if [ ! -d "$folder" ]; then
    mkdir "$folder"
    echo "Folder $folder created."
fi

# Step 3: Build assets (Tailwind CSS)
print_step "Step 3:" "Build assets (Tailwind CSS)"
if [ "$1" == "--no-assets" ]; then
    echo "Assets won't be built"
else
    if ! command -v npx &> /dev/null; then
        print_error "npx has to be installed to build assets"
    else
        execute_command "npx tailwindcss@3.3.0 -i main.css -o release/assets/tailwind.css -c tailwind.config.js --minify"
        echo "Assets built"
    fi
fi

# Step 4: Copy favicon.ico and assets directory to the release folder
print_step "Step 4:" "Copy favicon.ico and assets directory to the release folder"
if [ "$2" == "--no-copy" ]; then
    echo "Copying disabled."
else
    print_command "cp favicon.ico $folder"
    cp favicon.ico "$folder"
    echo "favicon.ico copied to $folder"
fi

# Step 5: Build projects with wasm-pack
print_step "Step 5:" "Build projects with wasm-pack"
build_paths=('./ --target no-modules' 'services/toast_service/ --target no-modules' 'services/rmenu_service/ --target no-modules' 'services/chat_service/ --target no-modules' 'services/modal_service/ --target no-modules')
pids=()
for build_path in "${build_paths[@]}"; do
   
    execute_command "wasm-pack -q build $build_path --no-typescript --release --out-dir $(pwd)/$folder --no-pack &"
    pids+=("$!")
done

# Step 6: Wait for wasm-pack build jobs to complete
print_step "Step 6:" "Wait for wasm-pack build jobs to complete"
echo "Waiting for wasm-pack build jobs to complete. Press Ctrl+C to abort..."
wait "${pids[@]}"
echo "All wasm-pack build jobs completed."

# Record the end time
end_time=$(date +%s)

# Calculate and display the time spent on execution
execution_time=$((end_time - start_time))
echo -e "\033[0;32mExecution completed in $execution_time seconds.\033[0m"
cleanup_and_exit 0

