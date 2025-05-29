<!-- markdownlint-disable MD033 MD041 -->

<p align="center">
  <img src="assets/odometer.png" alt="Odometer" width="250"/>
</p>
<p align="center">
  <h1 align="center">Odometer</h1>
</p>
<p align="center">
    A tool for benchmarking Ethereum clients.
</p>

> [!Warning]
> This tool is under active development and the benchmarking methodology has not yet been thoroughly reviewed. Results should be interpreted with caution.

## Prerequisites

Docker required.

## Usage

- Start up docker cli
- Run the binary using one of these commands:

  ```sh
  cargo run                    # Run tests for all clients
  cargo run --client geth      # Run tests for geth only
  cargo run --client reth      # Run tests for reth only
  ```

  You can use `-c` as a shorter alternative to `--client`.

## Documentation

Detailed documentation about the project is available in the `docs/` directory:

- [Architecture Overview](docs/architecture.md) - Overview of how the project is structured.
- [Design Decisions](docs/design-philosophy.md) - Reasoning the CLI design
- [Benchmarks](docs/benchmarks.md) - Available benchmarks
- [Extending Odometer](docs/extending-odometer.md) - Guide for adding new tests and supporting additional Ethereum clients

## License

MIT/APACHE

## Acknowledgements

The idea to use engine api and benchmark engine api requests was from [Nethermind's gas benchmarking tool](https://github.com/NethermindEth/gas-benchmarks)
