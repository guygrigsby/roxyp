ECHO_RUNNING = $(strip $(shell docker ps --format '{{.Image}}' | grep guygrigsby\/echo))

run: echo
	RUST_LOG=debug cargo run
echo:
ifeq (${ECHO_RUNNING},)
	docker run -d -p 8080:8080 guygrigsby/echo
endif

.PHONY: echo run
