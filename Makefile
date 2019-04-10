build:
	cargo build --release
install: build
	sudo cp target/release/onefetch /usr/local/bin
uninstall:
	sudo rm -f /usr/local/bin/onefetch
clean:
	cargo clean
