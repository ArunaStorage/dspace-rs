# DSpace-RS

dspace-rs is a Rust-based framework for managing and utilizing dataspaces. It consists of multiple crates, each providing specific functionalities, as well as documentation and test resources.

## Project Structure

### crates/

The `crates/` directory contains the individual Rust crates of the project:

- **edc_api**: Provides the API for the Eclipse Dataspace Components (EDC).
- **edc_client**: Implements the client for communicating with the EDC API.
- **edc_server**: Implements the server that provides the EDC API. (**Coming**)
- **odrl**: Offers support for the Open Digital Rights Language (ODRL) specification.

### docs/

The `docs/` directory contains the project documentation. Here you can find guides, architecture overviews, and other important information for using and developing the project.

### tests/

The `tests/` directory contains the test cases for the entire project. These tests are crucial for ensuring the functionality and integrity of the code.

Some tests run against a running instance of EDC.

You can use docker compose to start an instance for testing.

```
docker compose -f test_utils/docker-compose.yml up -d
cargo test 
```

The tests setup was mostly derived by the Typescript client [edc-connector-client](https://github.com/Think-iT-Labs/edc-connector-client) by [Think-it](https://think-it.io/).

## Installation

Instructions for installing and using the individual crates can be found in the respective README files within the `crates/` directory.

## Contributing

Contributions to the project are welcome! Contributions to this project are welcome! Whether you find bugs, want to request features, or submit enhancements, please feel free to open an issue or submit a pull request. For major changes, it's recommended to discuss them first to ensure alignment with project goals.
Please read the [`CODE OF CONDUCT`](CODE_OF_CONDUCT.md) to learn more about our guidelines and the contribution process.

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option. Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Synevi by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Contact

For inquiries or support regarding this project, you can reach out to the maintainers through GitHub issues.
