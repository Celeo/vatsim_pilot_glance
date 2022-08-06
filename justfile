default: run

run:
	@cargo r -- -d 5 KSAN

build:
	@cargo b

release-linux:
	@cargo b --release

release-windows:
	@cargo b --release --target x86_64-pc-windows-gnu
	-cd target/release/ && explorer.exe
	-cd target/x86_64-pc-windows-gnu/release/ && explorer.exe .

build-all: build release-linux release-windows
