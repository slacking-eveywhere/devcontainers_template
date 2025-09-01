#!/usr/bin/env bash

PROJECTS_TOP_DIR="$HOME/.local/share/devcontainer/"
PROJECTS_BIN_DIR="$HOME/.local/bin"

TEMPLATE_FOLDER="$PROJECTS_TOP_DIR"/templates

mkdir -p "$TEMPLATE_FOLDER"
cat <<-EOF >"$TEMPLATE_FOLDER"/docker-compose-ssh.yml
	services:
	    maindevcontainer:
	        image: '${DEVCONTAINER_IMAGE_NAME}'
	        env_file:
	            - .env
	        volumes:
	            - workdir:/workdir
	            - home:/home
	        tmpfs: /tmp:exec,mode=1777
	        tty: true
	        stdin_open: true
	        command: ["sleep", "infinity"]
	        deploy:
	            resources:
	                reservations:
	                    memory: '${RESOURCES_RAM:-2G}'
	                limits:
	                    memory: '${RESOURCES_RAM:-2G}'

	volumes:
	    home:
	    workdir:
EOF

cat <<-EOF >"$TEMPLATE_FOLDER"/docker-compose-vscode.yml
	services:
	    app:
	        image: '${DEVCONTAINER_IMAGE_NAME}'
	        env_file:
	            - .env
	        volumes:
	            - ../:/workdir:cached
	            - home:/home
	            - type: tmpfs
	              target: /tmp
	        network_mode: host
	        tty: true
	        stdin_open: true
	        command: ["sleep", "infinity"]
	        deploy:
	            resources:
	                reservations:
	                    memory: '${RESOURCES_RAM:-2G}'
	                limits:
	                    memory: '${RESOURCES_RAM:-2G}'

	volumes:
	    home:
EOF

cat <<-EOF >"$TEMPLATE_FOLDER"/devcontainer.json
	{
	    "name": "changeme",
	    "dockerComposeFile": ["docker-compose.yml"],
	    "workspaceFolder": "/workdir",
	    "service": "app",
	    "customizations": {
	      "vscode": {
	          "forwardPorts": [
	            4321
	          ],
	        "settings": {
	          "remote": {
	            "restoreForwardedPorts": true,
	            "localPortHost": "0.0.0.0"
	          }
	        }
	      }
	    }
	  }
EOF

touch "$TEMPLATE_FOLDER"/.env

cp devcontainer.sh "$PROJECTS_BIN_DIR"/devcontainer
