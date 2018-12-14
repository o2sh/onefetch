install:
	cargo build --release
	sudo cp target/release/onefetch /usr/local/bin
uninstall:
	sudo rm -f /usr/local/bin/onefetch
