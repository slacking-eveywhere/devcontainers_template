#!/usr/bin/env bash

#
# Uninstallation script for devcontainer tool
#
# This script removes the devcontainer tool from:
#   - Binary: ~/.local/bin/devcontainer
#   - Data:   ~/.local/share/devcontainer/
#

set -o errexit
set -o nounset
set -o pipefail

# --- Constants ---
declare -r INSTALL_BIN="${HOME}/.local/bin/devcontainer"
declare -r INSTALL_SHARE_DIR="${HOME}/.local/share/devcontainer"

# --- Main Uninstallation ---
main() {
	local files_removed=0
	
	# Remove binary
	if [[ -f "${INSTALL_BIN}" ]]; then
		rm -f "${INSTALL_BIN}"
		files_removed=$((files_removed + 1))
	fi
	
	# Remove data directory
	if [[ -d "${INSTALL_SHARE_DIR}" ]]; then
		# Check for existing projects
		local projects_dir="${INSTALL_SHARE_DIR}/projects"
		if [[ -d "${projects_dir}" ]] && [[ -n "$(ls -A "${projects_dir}" 2>/dev/null)" ]]; then
			echo ""
			echo "WARNING: Projects exist in ${projects_dir}"
			read -r -p "Delete all projects and data? [y/N] " response
			case "${response}" in
				[yY][eE][sS]|[yY]) 
					rm -rf "${INSTALL_SHARE_DIR}"
					files_removed=$((files_removed + 1))
					;;
				*)
					echo "Keeping ${INSTALL_SHARE_DIR}"
					echo "To remove manually: rm -rf ${INSTALL_SHARE_DIR}"
					;;
			esac
		else
			rm -rf "${INSTALL_SHARE_DIR}"
			files_removed=$((files_removed + 1))
		fi
	fi
	
	if [[ ${files_removed} -eq 0 ]]; then
		echo "Nothing to remove (already uninstalled)"
	else
		echo "Uninstallation complete"
	fi
}

main "$@"
