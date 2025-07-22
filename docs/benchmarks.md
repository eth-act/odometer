# Supported Benchmarks

## Block Gas Limit Benchmarks

Currently, Odometer supports block gas limit benchmarks, which measure how efficiently clients process blocks with different gas limits. These benchmarks:

- Measure client performance in terms of gas processed per second
- Test various block gas limits (from 30M to 160M)
- Compare client performance as block gas limits increase
- Use standardized Engine API requests to ensure fair comparison
- Provide insights into block execution scaling characteristics

The block gas limit benchmarks execute through Engine API calls that simulate block production and execution at different gas limits, helping identify performance bottlenecks and optimization opportunities as block sizes grow.

## Adding New Benchmark Types

To add a new benchmark type:

1. Define the benchmark methodology and metrics
2. Create JSON test definition files in a new directory under `tests/`
3. Implement the necessary measurement logic in the Rust codebase
4. Add appropriate command-line interface options
5. Document the benchmark specifics in this file

For detailed instructions on extending Odometer with new benchmarks, see the [Extending Odometer](extending-odometer.md) guide.
