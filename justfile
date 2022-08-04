default: run

run:
	@cargo r -- -d 5 KSAN

release-windows:
	@cargo b --release --target x86_64-pc-windows-gnu
	@cp target/x86_64-pc-windows-gnu/release/vatsim_pilot_glance.exe .
