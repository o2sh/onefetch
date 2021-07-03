build:
	cargo build --release --features=fail-on-deprecated

install:
	cargo install --path "." --features=fail-on-deprecated

uninstall:
	cargo uninstall onefetch

clean:
	cargo clean

release-mac:
	strip target/release/onefetch
	mkdir -p release
	tar -C ./target/release/ -czvf ./release/onefetch-mac.tar.gz ./onefetch

release-win:
	mkdir -p release
	tar -C ./target/release/ -czvf ./release/onefetch-win.tar.gz ./onefetch.exe

release-linux:
	strip target/release/onefetch
	mkdir -p release
	tar -C ./target/release/ -czvf ./release/onefetch-linux.tar.gz ./onefetch
