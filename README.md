# vatsim_pilot_glance

[![CI](https://github.com/Celeo/vatsim_pilot_glance/workflows/CI/badge.svg?branch=master)](https://github.com/celeo/vatsim_pilot_glance/actions?query=workflow%3ACI)

TUI to show VATSIM users near an airport and their piloting (and controlling) time on the network.

Intended use is to spot new pilots in order to plan extra time for them.

## Building

1. Install Rust
1. Clone <https://github.com/Celeo/vatsim_pilot_glance>
1. `cd vatsim_pilot_glance`
1. `cargo build`

## Installing

Either build from source, or get a binary from the [releases page](https://github.com/Celeo/vatsim_pilot_glance/releases).

## Using

Run the binary, passing in an airport code (KSAN, KLAS, etc.) as the only positional argument. You can also
specify the distance to look in relation to that airport (default is 20 nm).

The app will update every 15 seconds with new pilot locations (the refresh rate of the data made available through VATSIM).
No interaction is required to trigger these updates.

The up and down arrows can be used to select a row, then the `o` key to open that pilot's online stats page for more information.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE))
* MIT license ([LICENSE-MIT](LICENSE-MIT))

## Contributing

Please feel free to contribute. Please open an issue first (or comment on an existing one) so that I know that you want to add/change something.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
