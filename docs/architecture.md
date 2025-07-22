# Odometer Architecture

## Project structure

```sh
Cargo.toml                 # Workspace configuration
Cargo.lock                 # Locked dependencies
LICENSE-APACHE.txt         # Apache license
LICENSE-MIT.txt            # MIT license
README.md                  # Project documentation
📁 assets                  # Project assets
└── odometer.png          # Project logo
📁 clients                 # Docker files for clients
├── besu.yml
├── erigon.yml
├── geth.yml
├── nethermind.yml
└── reth.yml
📁 config                  # Configuration files
├── chainspec.json        # Chain specification
├── config.toml           # Project configuration
├── genesis.json          # Genesis configuration
├── jwt.hex               # JWT token
└── 📁 docker             # Docker configurations
    ├── common.yml        # Base docker service
    └── entrypoint.sh     # Docker entrypoint
📁 crates                  # Rust crates
└── 📁 profiler           # Benchmarking logic
    ├── Cargo.toml
    └── 📁 src
        ├── bench_summary.rs  # Payload interface
        ├── docker.rs         # Docker interface
        ├── engine_api.rs     # Engine API interface
        ├── kute.rs           # Client interface
        └── main.rs           # Entrypoint
📁 docs                    # Project documentation
├── architecture.md       # Architecture overview
├── benchmarking-methodology.md # Benchmarking methodology details
├── design-decisions.md   # Design decisions documentation
└── extending-odometer.md # Guide for extending the project
📁 nethermind_test_conversion # Conversion scripts for Nethermind tests
├── gaslimit.sh           # Conversion script
└── 📁 GasLimit           # Original Nethermind GasLimit test files
    ├── GasLimit_30M.txt
    ├── GasLimit_40M.txt
    └── ...
📁 tests                   # Test definitions
└── 📁 GasLimit           # Gas limit test files
    ├── GasLimit_30M.json
    ├── GasLimit_40M.json
    └── ...
```
