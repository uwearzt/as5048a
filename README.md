# [AS5048A](https://crates.io/crates/as5048a)

[![Apache licensed](https://img.shields.io/badge/license-Apache-blue.svg)](http://www.apache.org/licenses/LICENSE-2.0)
[![Build Status](https://travis-ci.org/uwearzt/as5048a.svg?branch=master)](https://travis-ci.org/uwearzt/as5048a)
[![crates.io](https://meritbadge.herokuapp.com/as5048a)](https://crates.io/crates/as5048a)

Rust driver for AMS AS5048A Magnetic Rotary Encoder

## Documentation

 Read the detailed documentation [here](https://docs.rs/as5048a/)

## Build

Rust nightly is needed to compile this crate.

### Raspberry Pi

```bash
cargo build --example raspberrypi
cargo run --example raspberrypi
```

## ToDo

- [ ] add complete API
- [ ] Travis CI
- [ ] API documentation
- [ ] refactoring

## Done

- [x] get the primary sensor readouts

## License

[Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)

## Resources

- [Datasheet](https://ams.com/as5048a)
