.PHONY: install build uninstall clean release-mac release-win release-linux release-deb

install:
	cargo install --path . --locked

build:
	cargo build --release --locked

uninstall:
	cargo uninstall onefetch

clean:
	cargo clean

release-mac:
	mkdir -p release
	tar -C ./target/release/ -czvf ./release/onefetch-mac.tar.gz ./onefetch

release-win: TAG_NAME = $$(git describe --abbrev=0 --tags)
release-win:
	mkdir -p release
	tar -C ./target/release/ -czvf ./release/onefetch-win.tar.gz ./onefetch.exe
	iscc.exe -DMyAppVersion=${TAG_NAME} ./.github/workflows/windows-installer.iss

release-linux:
	mkdir -p release
	tar -C ./target/release/ -czvf ./release/onefetch-linux.tar.gz ./onefetch

release-deb:
	mkdir -p release
	cargo deb --no-build --output ./release/onefetch_amd64.deb
