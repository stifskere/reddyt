set dotenv-load

@dev *flags:
	#!/bin/bash
	set -e;

	# Initialize variables to avoid UB.
	NO_BINSTALL=0;

	# It is difficult for a shell to unintentionally
	# inject a `;` to alter the behavior of this, since
	# `;` are usually treated to separate commands.
	#
	# In the case someone wanted to alter the behavior
	# of this, they would end up loosing.
	for flag in {{flags}}; do
		case $flag in
			# Use cargo install instead of binstall.
			--no-binstall)
				NO_BINSTALL=1;
			;;

			# Fallback.
			* )
			;;
		esac
	done

	if
		[[ -f /.dockerenv ]] ||
		grep -qE '(docker|containerd|kubepods)' /proc/1/cgroup 2>/dev/null;
	then
		# If we are inside a docker container we will run the commands
		# directly. We asume being inside one because of the existence
		# of ./.dockerenv or a group called either docker, containerd
		# or kubepods.

		cargo watch -x 'run -p backend' 2>&1 & :;
		trunk serve --config Trunk.toml 2>&1 & :;
		wait;
	else
		# On the user's computer, we will call docker to build the image.

		if !command -v docker >/dev/null 2>&1; then
			echo "Please, install a linux compatible version of docker before continuing.";
			exit 1;
		fi

		# Since `docker compose up` does not accept
		# build arguments we need to build explicitly.
		docker compose \
			-f ./docker/dev.docker-compose.yml build \
			--build-arg NO_BINSTALL="$NO_BINSTALL";

		docker compose \
			-f ./docker/dev.docker-compose.yml up \
			--no-deps;
	fi
