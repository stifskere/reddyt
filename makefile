.ONESHELL:
.PHONY: dev

dev:
	PIDS="";

	cleanup() {
		for pid in $$PIDS
		do
			kill $$pid 2>/dev/null || true;
		done
		exit 0;
	}

	trap cleanup SIGINT SIGTERM;

	(cd frontend && trunk serve --open 2>&1 | awk '{print "[frontend] "$$0}') &
	PIDS="$$PIDS $$!";

	cargo run -p backend | awk '{print "[backend] "$$0}' &
	PIDS="$$PIDS $$!";

	wait;
