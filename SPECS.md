# Devcontainers specs

This project provides devcontainers for IDEs. It is a CLI tool that allows you to create, edit, stop, and list devcontainer projects.  
A project is a .env file with project settings, a docker-compose.yml file and a volumes folder in user-space (like ~/.local/devcontainers/<name>/{vol1, vol2, ...})

A project can run on the local docker context or on a remote context. It does not use docker context but instead proxies through an SSH tunnel. The SSH tunnel is created and deleted on each command if the target is a remote host.

## Structures

### .env file template

```bash
# --- User Configuration ---
# Set the username and UID for the devcontainer user.
USER=
USER_ID=

# --- Devcontainer Image ---
# Specify the Docker image to use for the development environment.
DEVCONTAINER_IMAGE_NAME=

# --- Resource Limits ---
# Optional: Define resource limits, e.g., RAM.
RESOURCES_RAM=
# --- SSH Configuration ---
# Optional: Define SSH port for accessing the devcontainer.
HOSTNAME=
SSH_PORT=
ENABLE_SSH=true
```

### docker-compose.yml

```yaml
# Default docker-compose default file for devcontainer
# Services declaration
services:
  maindevcontainer:
    image: "${DEVCONTAINER_IMAGE_NAME:-bash:5}"
    env_file:
      - .env
    volumes:
      - workdir:/workdir
      - home:/home/${USER}
      - ${DOCKER_SOCK:-/var/run/docker.sock}/var/run/docker.sock
      - type: tmpfs
        target: /tmp
    ports:
      - "${SSH_PORT:-22224}:22"
    tmpfs: /tmp:exec,mode=1777
    tty: true
    stdin_open: true
    command: ["sleep", "infinity"]
    deploy:
      resources:
        reservations:
          memory: "${RESOURCES_RAM:-2G}"
        limits:
          memory: "${RESOURCES_RAM:-2G}"
          
# Volumes declaration
volumes:
    home:
    workdir:

```

### docker-compose.extended.yml

```yaml
# docker compose file that can be modified to add some specs to the dev stack
services:
  maindevcontainer:
    # This service inherits its base configuration from the template.
    # You can add overrides here, such as volume mounts or port mappings.
    env_file:
      - ${PROJECTS_TOP_DIR}/${project_name}/.env
```

### devcontainer.json

```json
{
  "name": "changeme",
  "dockerComposeFile": ["docker-compose.yml"],
  "workspaceFolder": "/workdir",
  "service": "app",
  "customizations": {
    "vscode": {
      "forwardPorts": [4321],
      "settings": {
        "remote": {
          "restoreForwardedPorts": true,
          "localPortHost": "0.0.0.0",
        },
      },
    },
  },
}
```

## CLI args

`name` is a string and is mandatory whenever it is required.  

- `new <name>` Create a new project with the given name.
  1. If a name is provided, create a new project with that name.
  2. If a project with that name already exists, display an error stating that the project name already exists.
  3. `-f` or `--force` as an optional argument to overwrite the project.
  4. If no name is provided, display an error stating that a name is required.
  5. If a name is provided and `-f` or `--force` is provided but the project does not exist yet, create a new project.
  6. If a name is provided and `-f` or `--force` is provided and a project with that name exists, overwrite the project with that name.

- `edit <name>` Edit a project with the given name.
  1. If a name is provided, open the configuration file in the editor.
  2. If a name is not provided, display an error stating that the name is mandatory.

- `inspect <name>` Inspect project configuration and status.
  1. If a name is provided, display the project configuration.
  2. If a name is not provided, display an error stating that the name is mandatory.
  3. Inspection is a formatted display of the configuration and status of a project.

- `run <name>` Run a docker-compose project with settings.
  1. If a name is provided, run docker-compose with the settings.
  2. If a name is not provided, display an error.
  3. Running must finish without errors; if errors occur, a summary must explain why the errors happened.
  4. Running must display a summary of what happened.
  5. Running must return to the terminal and start in the background.
  6. If `-s <key path>` is provided, pass the SSH key to the start as a docker cp command to populate the local user home directory `.ssh/authorized_keys`. The public key must be validated as a public key before copying it. If the key is not a valid public key, display an error.

- `stop <name>` Stop a container project.
  1. If a name is provided, stop the container.
  2. If a name is not provided, display an error.
  3. Stopping a container must finish without errors.
  4. If `-d` or `--detach` is provided, run the command and return immediately.
  5. If `-d` or `--detach` is not provided, run the command and wait for the container to stop.
  6. If `-f` or `--force` is provided, use the docker stop force option to kill the container.

- `connect <name>` Connect to a container via SSH.
  1. If a name is provided, read the port from the .env configuration and connect to it.
  2. If a name is not provided, display an error.
  3. Connect to SSH without asking for a password, using only a private key.
  4. If no private key is provided, test with the local agent.
  5. If `-s` key argument is passed, use that key.
  6. If a connection error occurs, display an error.
  7. Parse the .env file to get the SSH host and port; if one is missing, use the default but display a warning about the use of default values.
  8. If the .env file parsing fails, display an error.

- `list` List all projects.
  1. If no projects exist, display an empty table.
  2. If projects exist, display them as a table.
  3. The list table must display: project name `name`, status (running, stopped, on errors), configured ports, and configured image name.

- `help` Display the CLI help with a summary for each command and subcommand, and optional arguments.

- `version` Display the version of the CLI.

## Workflow

### 1. First-time local project setup

This is the most common entry point. A developer creates a brand new project, configures it, starts it, connects to it, and eventually stops it.

```bash
# Create a new project named "myproject"
devcontainer new myproject

# Open the .env configuration file in the default editor to set the image,
# user, SSH port, and resource limits
devcontainer edit myproject

# Start the project in the background
devcontainer run myproject

# Verify the project is running and review its configuration
devcontainer inspection myproject

# Open an interactive SSH session inside the container
devcontainer connect myproject

# Stop the container and wait for it to shut down cleanly
devcontainer stop myproject
```

### 2. Reinitialising an existing project

A developer wants to reset a project that already exists — for example, to apply a new base image — without having to delete it manually first.

```bash
# List all existing projects to confirm the project exists
devcontainer list

# Overwrite the existing "myproject" configuration with a fresh one
devcontainer new --force myproject

# Edit the configuration to update the image or settings
devcontainer edit myproject

# Restart the project with the new configuration
devcontainer run myproject

# Confirm the new configuration is active
devcontainer inspection myproject
```

### 3. Remote host project with SSH key injection

A developer wants to run a devcontainer on a remote machine. The project is configured with a remote hostname, and an SSH public key is injected into the container at startup so that passwordless access is available immediately.

```bash
# Create a new project intended for a remote host
devcontainer new remote-project

# Edit the .env to set HOSTNAME to the remote machine address and
# configure the desired SSH_PORT
devcontainer edit remote-project

# Start the project on the remote host and inject the developer's
# SSH public key into the container's authorized_keys
devcontainer run remote-project -s ~/.ssh/id_ed25519.pub

# Connect to the remote container using the matching private key
devcontainer connect remote-project -s ~/.ssh/id_ed25519

# Stop the remote container immediately without waiting
devcontainer stop remote-project --detach
```

### 4. Daily developer routine

A developer returns to an already configured project. They check the state of all their projects, inspect the one they want to work on, start it, and connect to it.

```bash
# Get an overview of all projects and their current statuses
devcontainer list

# Inspect the target project to review its configuration before starting
devcontainer inspection myproject

# Start the project (it was stopped from the previous session)
devcontainer run myproject

# Connect to the running container via SSH using the local SSH agent
devcontainer connect myproject

# At the end of the session, force-kill the container if it is unresponsive
devcontainer stop myproject --force
```

### 5. Recovering from a misconfigured project

A developer notices a project is in an error state after a failed start. They inspect it to understand the problem, force-reset the configuration, and bring it back up cleanly.

```bash
# List projects to spot the one in an error state
devcontainer list

# Inspect the failing project to read its current configuration and status
devcontainer inspection broken-project

# Overwrite the broken project with a clean configuration
devcontainer new --force broken-project

# Edit the fresh configuration to fix the image name or resource settings
devcontainer edit broken-project

# Attempt to run the project again and read the startup summary for errors
devcontainer run broken-project

# Once running, verify the project is healthy
devcontainer inspection broken-project
```
## Software pipeline

### 1. Create new project
- check if project with name exists
- if project exists and `force` argument is not passed, throw error
- if project exits and `force` argument is passed OR project does not exists, create the projet.
- create folder and template skell for `.env` file, `docker-compose.yml` file or `devcontainer.json` file.

### 2. Edit a project
- Check if if project with name exists, if not throw error else continue
- Open `.env` file if no argument is passed.
- if `-c` is passed, open the `docker-compose.yml` file.
- The editor is the default xdg editor in terminal or desktop software like code, zed, whatever
- if the `-q` argument is passed, do not reload the container, else reload container with new configuration

### 3.
