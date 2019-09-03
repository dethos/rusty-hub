# Rusty hub

This project started as a learning exercice and as a secondary goal it intends to be an easy to use and deploy [Websub Hub](https://www.w3.org/TR/websub/#hub).

It still is a work in progress and at this moment it is still not ready to be used.

**CI Status**: [![CircleCI](https://circleci.com/gh/dethos/rusty-hub.svg?style=svg)](https://circleci.com/gh/dethos/rust-hub)

## Missing features

Below are some missing features that are required for the hub to work properly:

- [ ] Subscription validation
- [ ] Subscription expiration
- [ ] Content distribution
- [ ] Authenticated content distribution
- [ ] Change default settings through a `config` file

## Development Environment

To start hacking on this project you just need to have your stable rust environment set up and then:

1. Clone this repository.
2. Run `cargo build` to build the project and install the dependencies.
3. Run `cargo test` to run the test suite and be sure everything is working properly
4. Run `cargo run` to manually test
