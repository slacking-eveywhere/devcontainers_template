# Bash Scripting Guidelines

## Philosophy

Write for bash 5+. Target modern bash features and idioms. Avoid ksh, zsh, and legacy sh syntax. Write scripts that are readable, maintainable, and robust.
If bash file is a docker-entrypoint or a dependencie of docker-entrypoint, refer to Docker-Image.md guidelines and stop reading this guideline.

---

## Shebang and Shell Declaration

Always use the bash shebang:

```bash
#!/usr/bin/env bash
```

**Never use:**
- `#!/bin/sh` — invokes POSIX sh, not bash
- `#!/bin/bash` — hardcodes the path, breaks on systems where bash is elsewhere
- `#!/usr/bin/env sh` — same as `/bin/sh`
- `#!/bin/zsh` or `#!/bin/ksh` — different shells with incompatible syntax

---

## Strict Mode

Start every script with these safety options:

```bash
set -euo pipefail
```

**What this does:**
- `-e` — exit immediately if any command fails
- `-u` — treat unset variables as errors
- `-o pipefail` — return the exit code of the first failed command in a pipeline

**Optional additions:**
- `set -x` — print each command before executing (useful for debugging)
- `IFS=$'\n\t'` — set the internal field separator to newline and tab only (prevents word splitting on spaces)

---

## Variables

### Naming

- **Local variables**: lowercase with underscores (`my_variable`)
- **Global variables**: uppercase with underscores (`MY_GLOBAL`)
- **Environment variables**: uppercase with underscores (`PATH`, `HOME`)
- **Constants**: uppercase with `readonly` (`readonly MAX_RETRIES=3`)

### Declaration

Always declare variables with proper scope:

```bash
# Local variable in a function
local my_var="value"

# Global variable
MY_GLOBAL="value"

# Read-only constant
readonly API_URL="https://example.com"

# Integer variable (bash 5+)
declare -i count=0

# Array
declare -a my_array=("item1" "item2" "item3")

# Associative array (bash 4+)
declare -A my_map=(["key1"]="value1" ["key2"]="value2")
```

### Quoting

**Always quote variables** unless you explicitly need word splitting:

```bash
# Good
echo "$my_variable"
cp "$source" "$destination"

# Bad (word splitting, globbing)
echo $my_variable
cp $source $destination

# Exception: when you want word splitting (rare)
args="--verbose --output file.txt"
command $args  # intentional word splitting
```

### Parameter Expansion

Use modern bash parameter expansion instead of external tools:

```bash
# Default values
echo "${var:-default}"        # use default if var is unset or empty
echo "${var:=default}"        # assign default if var is unset or empty

# String manipulation
echo "${var#prefix}"          # remove shortest prefix match
echo "${var##prefix}"         # remove longest prefix match
echo "${var%suffix}"          # remove shortest suffix match
echo "${var%%suffix}"         # remove longest suffix match
echo "${var/pattern/replace}" # replace first match
echo "${var//pattern/replace}" # replace all matches

# Case conversion (bash 4+)
echo "${var^^}"               # uppercase
echo "${var,,}"               # lowercase
echo "${var^}"                # capitalize first letter

# Length
echo "${#var}"                # string length

# Substring
echo "${var:0:5}"             # first 5 characters
echo "${var:5}"               # from position 5 to end
```

---

## Arrays

### Indexed Arrays

```bash
# Declaration
declare -a fruits=("apple" "banana" "cherry")

# Append
fruits+=("date")

# Access element
echo "${fruits[0]}"

# All elements
echo "${fruits[@]}"

# Number of elements
echo "${#fruits[@]}"

# Iterate
for fruit in "${fruits[@]}"; do
    echo "$fruit"
done

# Iterate with index
for i in "${!fruits[@]}"; do
    echo "$i: ${fruits[$i]}"
done
```

### Associative Arrays (bash 4+)

```bash
# Declaration
declare -A config=(
    ["host"]="localhost"
    ["port"]="8080"
    ["debug"]="true"
)

# Access
echo "${config[host]}"

# All keys
echo "${!config[@]}"

# All values
echo "${config[@]}"

# Iterate
for key in "${!config[@]}"; do
    echo "$key = ${config[$key]}"
done

# Check if key exists
if [[ -v config[host] ]]; then
    echo "host is set"
fi
```

---

## Conditionals

### Test Syntax

**Always use `[[ ]]` instead of `[ ]` or `test`:**

```bash
# Good (bash built-in, more features, safer)
if [[ "$var" == "value" ]]; then
    echo "match"
fi

# Bad (POSIX, less features, requires more quoting)
if [ "$var" = "value" ]; then
    echo "match"
fi
```

### Comparison Operators

```bash
# String comparison
[[ "$a" == "$b" ]]    # equal
[[ "$a" != "$b" ]]    # not equal
[[ "$a" < "$b" ]]     # lexicographic less than
[[ "$a" > "$b" ]]     # lexicographic greater than
[[ -z "$a" ]]         # string is empty
[[ -n "$a" ]]         # string is not empty

# Numeric comparison
[[ "$a" -eq "$b" ]]   # equal
[[ "$a" -ne "$b" ]]   # not equal
[[ "$a" -lt "$b" ]]   # less than
[[ "$a" -le "$b" ]]   # less than or equal
[[ "$a" -gt "$b" ]]   # greater than
[[ "$a" -ge "$b" ]]   # greater than or equal

# File tests
[[ -e "$file" ]]      # exists
[[ -f "$file" ]]      # is a regular file
[[ -d "$dir" ]]       # is a directory
[[ -r "$file" ]]      # is readable
[[ -w "$file" ]]      # is writable
[[ -x "$file" ]]      # is executable
[[ -s "$file" ]]      # file exists and is not empty

# Pattern matching
[[ "$var" == *.txt ]]         # glob pattern
[[ "$var" =~ ^[0-9]+$ ]]      # regex pattern

# Logical operators
[[ "$a" == "x" && "$b" == "y" ]]  # AND
[[ "$a" == "x" || "$b" == "y" ]]  # OR
[[ ! -f "$file" ]]                # NOT
```

### If-Elif-Else

```bash
if [[ "$status" == "active" ]]; then
    echo "System is active"
elif [[ "$status" == "inactive" ]]; then
    echo "System is inactive"
else
    echo "Unknown status"
fi
```

---

## Loops

### For Loop

```bash
# Iterate over list
for item in "one" "two" "three"; do
    echo "$item"
done

# Iterate over array
for item in "${array[@]}"; do
    echo "$item"
done

# C-style for loop
for ((i = 0; i < 10; i++)); do
    echo "$i"
done

# Iterate over files (safe, no globbing issues)
for file in ./*.txt; do
    [[ -e "$file" ]] || continue  # skip if no matches
    echo "$file"
done
```

### While Loop

```bash
# Basic while
while [[ "$count" -lt 10 ]]; do
    echo "$count"
    ((count++))
done

# Read lines from file
while IFS= read -r line; do
    echo "$line"
done < file.txt

# Read command output
while IFS= read -r line; do
    echo "$line"
done < <(command)
```

### Until Loop

```bash
until [[ "$status" == "ready" ]]; do
    status=$(check_status)
    sleep 1
done
```

---

## Functions

```bash
# Function declaration
my_function() {
    local arg1="$1"
    local arg2="${2:-default}"
    
    # Function body
    echo "arg1: $arg1, arg2: $arg2"
    
    # Return exit code
    return 0
}

# Call function
my_function "value1" "value2"

# Capture return value (exit code)
if my_function "test"; then
    echo "Success"
fi

# Capture output
result=$(my_function "test")
```

### Function Best Practices

- Always use `local` for function variables
- Use `return` for exit codes (0-255)
- Use `echo` or `printf` to return strings (capture with command substitution)
- Document parameters in comments
- Validate arguments

```bash
# Process a file with validation
# Arguments:
#   $1 - input file path
#   $2 - output file path (optional)
process_file() {
    local input_file="$1"
    local output_file="${2:-output.txt}"
    
    # Validate
    if [[ ! -f "$input_file" ]]; then
        echo "Error: input file does not exist" >&2
        return 1
    fi
    
    # Process
    grep "pattern" "$input_file" > "$output_file"
    return 0
}
```

---

## Command Substitution

**Use `$()` instead of backticks:**

```bash
# Good
result=$(command)
files=$(ls -1)

# Bad (legacy syntax, hard to nest)
result=`command`
files=`ls -1`

# Nested (easy with $())
result=$(echo "$(date) - $(hostname)")
```

---

## Process Substitution

Use `<()` for commands that need file arguments:

```bash
# Compare output of two commands
diff <(command1) <(command2)

# Read from command output
while IFS= read -r line; do
    echo "$line"
done < <(command)
```

---

## Arithmetic

### Use `(( ))` for arithmetic operations:

```bash
# Assignment
((count = 5))
((count++))
((count += 10))
((result = count * 2 + 3))

# Comparison
if ((count > 10)); then
    echo "count is greater than 10"
fi

# In conditionals
while ((count < 100)); do
    ((count++))
done
```

### Alternative: `$(( ))`

```bash
# For inline arithmetic expansion
echo "Result: $((5 + 3))"
result=$((count * 2))
```

---

## String Operations

### Concatenation

```bash
# Simple concatenation
full_name="$first_name $last_name"

# Appending
path="/usr/local"
path="$path/bin"
```

### String Testing

```bash
# Empty or unset
[[ -z "$var" ]]       # true if empty or unset
[[ -n "$var" ]]       # true if not empty

# Pattern matching
[[ "$filename" == *.txt ]]
[[ "$email" =~ ^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$ ]]
```

---

## Input/Output

### Reading Input

```bash
# Read a line from stdin
read -r line

# Read with prompt
read -rp "Enter your name: " name

# Read into array (split on IFS)
read -ra words <<< "$sentence"

# Read securely (no echo, for passwords)
read -rsp "Password: " password
echo  # newline after password input
```

### Output

```bash
# Echo (simple, adds newline)
echo "Hello, world"

# Printf (formatted, more control)
printf "%s\n" "Hello, world"
printf "%-10s %5d\n" "Name" 42

# Write to stderr
echo "Error message" >&2

# Write to file
echo "content" > file.txt      # overwrite
echo "content" >> file.txt     # append
```

---

## Error Handling

### Exit on Error

```bash
# Exit with error message
error_exit() {
    echo "Error: $1" >&2
    exit 1
}

# Usage
[[ -f "$file" ]] || error_exit "File not found: $file"
```

### Trap

```bash
# Cleanup on exit
cleanup() {
    rm -f "$temp_file"
}
trap cleanup EXIT

# Handle signals
trap 'echo "Interrupted"; exit 130' INT
trap 'echo "Terminated"; exit 143' TERM
```

### Error Checking

```bash
# Check command success
if ! command arg1 arg2; then
    echo "Command failed" >&2
    exit 1
fi

# Or with explicit check
command arg1 arg2
if [[ $? -ne 0 ]]; then
    echo "Command failed with exit code $?" >&2
    exit 1
fi
```

---

## Subshells and Command Grouping

```bash
# Subshell (runs in separate process, changes don't affect parent)
(
    cd /tmp || exit
    command
)

# Command group (runs in current shell)
{
    cd /tmp || exit
    command
}
```

---

## Here Documents and Here Strings

### Here Document

```bash
# Multi-line string
cat <<EOF
This is a multi-line
string that preserves
    formatting
EOF

# With variable expansion
cat <<EOF
Hello, $name
Today is $(date)
EOF

# Without variable expansion (quote the delimiter)
cat <<'EOF'
$name will not be expanded
EOF

# Indented (use <<- and tabs)
if true; then
	cat <<-EOF
	This is indented
	EOF
fi
```

### Here String

```bash
# Single-line input
grep "pattern" <<< "$variable"

# Read into array
read -ra words <<< "$sentence"
```

---

## Common Pitfalls to Avoid

### 1. Unquoted Variables

```bash
# Bad
file=$1
cat $file  # breaks if filename has spaces

# Good
file="$1"
cat "$file"
```

### 2. Using `ls` for File Lists

```bash
# Bad
files=$(ls)
for file in $files; do
    echo "$file"
done

# Good
for file in ./*; do
    [[ -e "$file" ]] || continue
    echo "$file"
done
```

### 3. Testing `$?` After Multiple Commands

```bash
# Bad
command1
command2
if [[ $? -eq 0 ]]; then  # tests command2, not command1
    echo "Success"
fi

# Good
if command1; then
    echo "Success"
fi
```

### 4. Using `echo` for User Input

```bash
# Bad (code injection risk)
user_input="$1"
echo $user_input

# Good
printf "%s\n" "$user_input"
```

### 5. Using `cd` Without Checking

```bash
# Bad
cd /some/dir
rm -rf ./*  # dangerous if cd fails

# Good
cd /some/dir || exit 1
rm -rf ./*
```

---

## Modern Bash Features (bash 4+)

### Associative Arrays

See the Arrays section above.

### Case Modification

```bash
var="Hello World"
echo "${var^^}"  # HELLO WORLD
echo "${var,,}"  # hello world
echo "${var^}"   # Hello World
```

### Globstar (bash 4+)

```bash
shopt -s globstar

# Recursive glob
for file in **/*.txt; do
    echo "$file"
done
```

### Negative Array Indices (bash 4.3+)

```bash
array=("a" "b" "c" "d")
echo "${array[-1]}"  # d (last element)
echo "${array[-2]}"  # c (second-to-last)
```

### `mapfile` / `readarray` (bash 4+)

```bash
# Read lines into array
mapfile -t lines < file.txt

# From command output
mapfile -t lines < <(command)
```

---

## Style and Best Practices

### Naming Conventions

- Script names: lowercase with hyphens (`my-script.sh`)
- Functions: lowercase with underscores (`my_function`)
- Variables: lowercase with underscores (`my_variable`)
- Constants: uppercase with underscores (`MY_CONSTANT`)

### Indentation

- Use 4 spaces (not tabs)
- Indent function bodies
- Indent control structure bodies

### Line Length

- Keep lines under 100 characters
- Break long commands with `\`

```bash
command arg1 arg2 arg3 \
    arg4 arg5 arg6 \
    arg7 arg8
```

### Comments

```bash
# Single-line comment for simple explanations

# Multi-line comment for complex logic:
# This function does X by doing Y.
# It handles edge case Z by checking condition W.
my_function() {
    # ...
}
```

### Function and Variable Documentation

```bash
# Process configuration file
# Globals:
#   CONFIG_DIR
# Arguments:
#   $1 - config file name
# Returns:
#   0 on success, 1 on error
# Outputs:
#   Parsed configuration to stdout
process_config() {
    # ...
}
```

---

## Checklist

- [ ] Shebang is `#!/usr/bin/env bash`
- [ ] Script starts with `set -euo pipefail`
- [ ] All variables are quoted: `"$var"`
- [ ] Using `[[ ]]` instead of `[ ]`
- [ ] Using `$()` instead of backticks
- [ ] Functions use `local` for variables
- [ ] Arrays use proper syntax: `"${array[@]}"`
- [ ] No `ls` in for loops
- [ ] `cd` commands check for failure
- [ ] Errors go to stderr: `>&2`
- [ ] Functions validate their arguments
- [ ] Cleanup is handled with `trap`
