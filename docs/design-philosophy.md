# Design Philosophy

## Core Principles

Odometer follows these key design principles:

- **Clear Hierarchy**: Commands follow odometer → verb → noun pattern
- **Explicit Naming**: Self-explanatory verbs like measure, list, show, and nouns like gas, clients, payloads
- **Required Arguments First**: Essential parameters come first
- **Sensible Defaults**: Reasonable defaults for duration, tps, concurrency
- **Output Flexibility**: Multiple output formats for different needs
- **Extensibility**: Easy addition of new benchmark types
- **Discoverability**: Consistent help system

> [!NOTE]
> This document outlines the design philosophy and command structure for Odometer. Many of the commands described here serve as examples and may not yet be implemented in the current version of the tool.

## CLI Interface Design

- **Conversational**: Commands read like natural instructions
- **Discoverable**: Easy to find what you need with `help`
- **Extensible**: Seamlessly integrate new benchmark types
- **Consistent**: Predictable structure across different benchmark types

### Global Options

These options apply to all odometer commands:

```sh
odometer --version / -v     # Show application version
odometer --help / -h        # Display help information
odometer --debug            # Enable verbose debugging output
```

### Command Structure

The command structure follows the pattern: `odometer <verb> <noun> [options]`

```sh
odometer measure gas        # Measure gas consumption
odometer list clients       # List available clients
odometer show results       # Display benchmark results
```

## Command Reference

### 1. Global Commands

```sh
odometer help      # Shows main help message with available commands
odometer version   # Displays Odometer version information
odometer debug     # Enables verbose debugging output
```

### 2. Primary Actions

#### A. Measure Benchmarks

```sh
odometer measure gas [OPTIONS]
  --against <client1,client2,...>    # Required: Clients to benchmark
  --with <payload1,payload2,...>     # Required: Payloads to use
  --duration <seconds> / -d          # Optional: Test duration (default: 60)
  --rate <tps> / -r                  # Optional: Target transactions per second
  --output <format> / -o             # Optional: Output format (table, json, csv)
  --parallel <num_threads> / -p      # Optional: Concurrency level (default: 1)
  --config <file_path>               # Optional: Custom configuration file
  --endpoint <client>=<url>          # Optional: Override default RPC endpoint
```

Example:

```sh
odometer measure gas --against geth,erigon --with simple_tx --duration 120
```

#### B. List Available Resources

```sh
odometer list clients [OPTIONS]          # List configured Ethereum clients
odometer list payloads gas [OPTIONS]     # List available gas transaction payloads
```

Options:

```sh
  --verbose / -v     # Show detailed information
```

#### C. Show Detailed Information

```sh
odometer show results gas [run_id | --latest] [OPTIONS]   # Display gas benchmark results
odometer show client <client_name>                        # Show client configuration
```

Options:

```sh
  --output <format> / -o     # Specify output format
```

#### D. Configure System Components

```sh
odometer configure client add <name> --rpc-url <url> [OPTIONS]    # Add client
odometer configure client remove <name>                           # Remove client
odometer configure payload add gas <name> --from-file <path>      # Add payload
```

## Future Extensibility

The command structure allows easy addition of new benchmark types:

```sh
odometer measure zkevm --against geth --test-cases proof_generation_time
odometer list payloads zkevm
odometer show results zkevm
```
