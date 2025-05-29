# Extending Odometer

This guide explains how to extend Odometer with new test cases or add support for additional Ethereum clients.

## Adding New Ethereum Clients

### 1. Create Docker Configuration

Create a new YAML file in the `clients/` directory. Each client in Odometer uses a standard pattern for configuration:

```yaml
# Template for clients/your_client.yml
services:
  node:
    image: your-client-image:tag

    environment:
      # Command to initialize the client with genesis block
      INIT_COMMAND: |
        your-client --datadir=/data init /config/genesis.json

      # Command to run the client with Engine API enabled
      RUN_COMMAND: |
        your-client --nodiscover \
                   --datadir=/data \
                   --networkid=1700 \
                   --authrpc.addr=0.0.0.0 \
                   --authrpc.port=8551 \
                   --authrpc.vhosts=* \
                   --authrpc.jwtsecret=/config/jwt.hex

    # Extend from the common configuration
    extends:
      file: ../config/docker/common.yml
      service: base_client

    # Use the standard entrypoint script
    entrypoint: ["/bin/sh", "/entrypoint.sh"]
```

The configuration works through several key components:

1. **INIT_COMMAND**: An environment variable that specifies how to initialize the client with the genesis block. This command is executed once when the container starts.

2. **RUN_COMMAND**: An environment variable that specifies how to run the client with Engine API enabled. This command is executed after INIT_COMMAND completes.

3. **Entrypoint Script**: Located at `/config/docker/entrypoint.sh`, this script:
   - Removes any existing data directory
   - Runs the INIT_COMMAND to set up the genesis state
   - Runs the RUN_COMMAND to start the client
4. **Common Configuration**: The file `config/docker/common.yml` provides base configuration that all clients extend:
   - Sets up port mapping (8551:8551 for the Engine API)
   - Configures shared volumes (entrypoint script, JWT token, genesis configs)

### 2. Verify Client Compatibility

Ensure your client:

- Supports the Engine API
- Can authenticate using JWT
- Listens on port 8551 for Engine API requests
- Returns gas usage information in a compatible format

### 3. Test the Client

Run Odometer with your new client:

```sh
cargo run --client new_client
```
