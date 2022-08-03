default: run

run:
	@cargo r -- -d 5 KSAN

build-windows:
	@cargo b --release --target x86_64-pc-windows-gnu