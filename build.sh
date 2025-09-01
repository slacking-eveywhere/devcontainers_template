#!/usr/bin/env bash

PUSH=""
COMMON="false"
BAKE_FILE="docker/docker-bake.hcl"

while [[ "$#" -ne 0 ]]; do
	case "$1" in
	--language-name | -l)
		LANGUAGE_NAME="$2"
		shift 2
		;;
	--registry)
		REGISTRY="$2"

		if [[ "${REGISTRY: -1}" != "/" ]]; then
			REGISTRY="$REGISTRY"/
		fi
		shift 2
		;;
	--push | -p)
		PUSH="--push"
		shift 1
		;;
	--common | -c)
		COMMON="true"
		shift 1
		;;
	*)
		echo "Unknown parameter $1"
		exit 1
		;;
	esac
done

CMD=(docker buildx bake -f "$BAKE_FILE")
[ -n "$PUSH" ] && CMD+=("--push")

if [[ "$COMMON" == "true" ]]; then
	"${CMD[@]}" common
fi

if [[ -z "$LANGUAGE_NAME" ]]; then
	echo "No language name set"
	"${CMD[@]}"
else
	CMD+=("$LANGUAGE_NAME")
	"${CMD[@]}"
fi
