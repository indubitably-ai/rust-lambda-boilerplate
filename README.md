# Boltwall-Lambda-Rust

This repository contains the source code for a Rust-based AWS Lambda function. The function is designed to process events with a specific structure, as defined in the `Request` struct, and return a response in the form of a `Response` struct.

## Warning

Please be aware that the code in this repository is not production-ready. It is intended for development and testing purposes only. Users should exercise caution and perform thorough testing before considering it for a production environment.

## Getting Started

To get started with this project, clone the repository and ensure you have the Rust toolchain installed. Follow the steps below to set up your environment and begin using the Lambda function:

### Prerequisites

Ensure you have the following installed:
- Rust programming language: [Install Rust](https://www.rust-lang.org/tools/install)
- AWS CLI: [Install the AWS CLI](https://aws.amazon.com/cli/)
- Cargo Lambda: Install cargo-lambda with `cargo install cargo-lambda`

### Building the Project

To build the project, navigate to the project directory and run:
```
cd src
cargo lambda build --release --arm64 --output-format zip
```

### Running Tests

To run the tests:
```
cd src
cargo test
```

### Add Dependency

To add an external dependency to the project, navigate to the src directory and run:
```
cd src
cargo add X
```


### Plan

This command allows you to see the changes you're about to make to the terraform state, navigate to the project directory and run:
```
terraform plan
```


### Apply

This command allows you to apply the changes you're making to the terraform state, navigate to the project directory and run:
```
terraform apply
```
Confirm with "yes"