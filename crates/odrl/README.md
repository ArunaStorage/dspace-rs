# ODRL (Open Digital Rights Language) - Rust Implementation

ODRL is a widely recognized language for expressing permissions and constraints related to digital content usage. This project aims to provide a robust Rust implementation of ODRL, allowing developers to integrate ODRL capabilities into their applications with ease.

## Overview

This project is structured to provide comprehensive support for **ODRL version 2.2** specifications. It includes modules for handling various aspects of ODRL, such as defining actions, assets, policies, rules, and constraints. Additionally, it incorporates an evaluator for interpreting ODRL expressions, a JSON parser for handling ODRL documents in JSON format, and utilities for serialization and validation.


## Project Structure

- **src/model**: Defines the core model elements of ODRL, such as actions, assets, policies, rules, and constraints.
- **src/functions**: Houses the main functionality of the ODRL implementation, including parsers, serializers, validators, and a state machine for negotiations.
- **src/lib.rs**: Rust library entry point.
- **src/name_spaces.rs**: Namespace definitions used in the project.

## Features

- **Comprehensive ODRL Support**: Covers a wide range of ODRL specifications to accommodate diverse usage scenarios.
- **Modular Design**: Organized into distinct modules for clarity and maintainability.
- **JSON Support**: Provides utilities for parsing ODRL documents in JSON format.
- **Validation and Serialization**: Includes functionality for validating ODRL expressions and serializing them as needed.

## Usage

To utilize this ODRL Rust implementation in your project, follow these steps:

1. **Add as Dependency**: Include this project as a dependency in your `Cargo.toml`:

    ```toml
    [dependencies]
    odrl = "0.1.0"
    ```

2. **Import Modules**: Import the necessary modules in your Rust code to access ODRL functionalities (see the following example):

    ```rust
    use odrl::model::{Action, Asset, Policy, Rule};
    ```

3. **Integrate**: Integrate ODRL capabilities into your application logic according to your specific requirements.

For detailed usage examples and API documentation, refer to the provided documentation or explore the codebase directly.

## Contributing

Contributions to this project are welcome! Whether you find bugs, want to request features, or submit enhancements, please feel free to open an issue or submit a pull request. For major changes, it's recommended to discuss them first to ensure alignment with project goals.

## License

This project is licensed under the MIT License - see the [LICENSE](../../LICENSE) file for details.

## Acknowledgments

- This project builds upon the foundation of the ODRL specifications and previous implementations.

## Contact

For inquiries or support regarding this project, you can reach out to the maintainers through GitHub issues.
