platform-win-x86 = x86_64-pc-windows-gnu
platform-linux-x86 = x86_64-unknown-linux-gnu
platform-apple-x86 = x86_64-apple-darwin
platform-apple-arm = aarch64-apple-darwin

.PHONY: clean
clean:
	cargo clean

.PHONY: release
release:
	make clean
	@echo "start release.."
	@echo "$(platform-apple-arm)"
	@cargo build --release
	@echo "$(platform-linux-x86)"
	@cargo build --target=$(platform-linux-x86) --release
	@echo "$(platform-win-x86)"
	@cargo build --target=$(platform-win-x86) --release
	@echo "$(platform-apple-x86)"
	@cargo build --target=$(platform-apple-x86) --release
	@echo "release finished"
	make rename

.PHONY: rename
rename:
	@mv target/release/password-generator ./$(platform-apple-arm)-password-generator
	@mv target/$(platform-apple-x86)/release/password-generator ./$(platform-apple-x86)-password-generator
	@mv target/$(platform-win-x86)/release/password-generator.exe ./$(platform-win-x86)-password-generator.exe
	@mv target/$(platform-linux-x86)/release/password-generator ./$(platform-linux-x86)-password-generator
