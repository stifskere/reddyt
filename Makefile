.SILENT:

MAKEFILE_DIR := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))
CWD := $(abspath $(CURDIR))

.ONESHELL:
.SHELLFLAGS := -eu -c
SHELL := /bin/bash

MAKEFLAGS += --no-print-directory
ifneq ($(MAKEFILE_DIR),$(CWD))
.DEFAULT_GOAL := __redirect
__redirect:
	$(MAKE) -C $(MAKEFILE_DIR)
endif

.PHONY: __dev_container
__dev_container:
	PIDS="";

	cleanup() {
		for pid in $$PIDS
		do
			kill $$pid 2>/dev/null || true;
		done
		exit 0;
	}

	trap cleanup SIGINT SIGTERM;

	(cd frontend && trunk serve --config=./Trunk.toml 2>&1 | awk '{print "[frontend] "$$0}') &
	PIDS="$$PIDS $$!";
	echo "started frontend.";

	(cargo watch -x 'run -p backend' | awk '{print "[backend] "$$0}') &
	PIDS="$$PIDS $$!";
	echo "started backend."

	wait;

dev:
	if ! command -v docker >/dev/null 2>&1
	then
		echo "Please install a linux compatible version of docker before continuing.";
		exit 1;
	fi

	docker compose -f ./cd/dev.docker-compose.yml up $(COMPOSE_ARGS);
