
run: echo
	RUST_LOG=debug cargo run

echo:
	docker run -d -p 8080:8080 guygrigsby/echo
