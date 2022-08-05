default: run

run:
	@cargo r -- -d 5 KSAN

build:
	@cargo b

release-windows:
	@cargo b --release --target x86_64-pc-windows-gnu
	@cp target/x86_64-pc-windows-gnu/release/vatsim_pilot_glance.exe .
	-explorer.exe .

build-all: build release-windows
