use crate::bench_summary::{BenchEngineAPIRequestSummary, BenchInput};
use crate::kute::JwtClient;
use cli_table::{print_stdout, Cell, Style, Table};
use indicatif::ProgressIterator;
use std::fs;

pub async fn run() {
    // Parse Engine API requests that we want to benchmark
    let mut bench_inputs = read_gas_limit_files("tests/GasLimit").unwrap();
    // Sort the inputs by name, so they are printed in natural lexicographical order
    bench_inputs.sort_by(|a, b| natural_lexical_cmp(&a.name, &b.name));

    let mut rows = Vec::new();
    let mut header = vec!["Name".cell().bold(true), "Description".cell().bold(true)];

    // Get all available clients, filtered by command line argument if provided
    let clients: Vec<String> = get_clients();

    if clients.is_empty() {
        eprintln!("Error: No clients found in clients directory");
        std::process::exit(1);
    }

    // push the client names to the header
    for client in &clients {
        header.push(client.cell().bold(true));
    }

    // Create a matrix to store results: [test_index][client_index]
    let mut results = vec![vec![String::new(); clients.len()]; bench_inputs.len()];

    let total_clients = clients.len();

    // Outer loop over clients
    for (client_idx, client) in clients.iter().enumerate() {
        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!(
            "  Processing [{}/{}] {}",
            client_idx + 1,
            total_clients,
            client
        );
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

        println!("🐳 Starting docker container...");
        let dc = crate::docker::DockerCompose::new(&format!("{}.yml", client));
        dc.up().unwrap();

        println!("Waiting for client to be ready...");
        // Wait for the client to be ready with a 30 second timeout
        if let Err(e) = dc.wait_for_healthy(30).await {
            eprintln!("Failed to start client {}: {}", client, e);
            println!("Stopping docker container...");
            dc.down().unwrap();
            continue;
        }
        println!("Client is ready!");

        // Inner loop over all benchmarks for this client
        println!("Running benchmarks for {}...", client);
        for (test_idx, bench_input) in bench_inputs.iter().enumerate().progress() {
            let summary = benchmark_engine_api_request(&bench_input).await;

            // Store the result for this test and client
            for sum in summary {
                let gas_used_u128 = parse_gas_used(&sum.gas_used);
                let gas_per_second =
                    compute_gas_per_second(gas_used_u128, sum.time_taken_microseconds);
                results[test_idx][client_idx] = format_gas_rate(gas_per_second);
            }
        }

        println!("Stopping docker container for {}...", client);
        dc.down().unwrap();
    }

    // Build the final table rows from the results matrix
    for (idx, bench_input) in bench_inputs.iter().enumerate() {
        let mut row = vec![
            bench_input.name.clone().cell(),
            bench_input.description.clone().cell(),
        ];

        // Add all client results for this test
        for client_result in &results[idx] {
            row.push(client_result.cell());
        }

        rows.push(row);
    }

    let table = rows.table().title(header).bold(true);
    assert!(print_stdout(table).is_ok());
}

fn get_clients() -> Vec<String> {
    let client_files = fs::read_dir("clients").unwrap();
    let mut clients = Vec::new();
    for client_file in client_files
        .filter_map(Result::ok)
        .filter(|client_file| client_file.path().extension().unwrap() == "yml")
    {
        let client_name = client_file
            .file_name()
            .to_str()
            .unwrap()
            .trim_end_matches(".yml")
            .to_string();
        clients.push(client_name);
    }
    clients
}

fn read_gas_limit_files(path_to_dir: &str) -> Result<Vec<BenchInput>, Box<dyn std::error::Error>> {
    fs::read_dir(path_to_dir)?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "json"))
        .map(|entry| {
            let content = fs::read_to_string(entry.path())?;
            Ok(serde_json::from_str(&content)?)
        })
        .collect()
}

async fn benchmark_engine_api_request(
    bench_input: &BenchInput,
) -> Vec<BenchEngineAPIRequestSummary> {
    // Setup Engine API credentials
    let secret: Vec<u8> =
        hex::decode(fs::read_to_string("config/jwt.hex").unwrap().trim()).unwrap();
    let client = JwtClient::new(secret, "http://localhost:8551".to_string());
    let mut summaries = Vec::new();

    for sequence_element in &bench_input.sequence {
        let timed_response = match client.send_request(&sequence_element.request).await {
            Ok(timed_response) => timed_response,
            Err(err) => {
                println!("fail {}", err);
                continue;
            }
        };

        // Check if we should keep this measurement
        if sequence_element.expect_measurement {
            let summary = BenchEngineAPIRequestSummary {
                description: sequence_element.description.clone(),
                time_taken_microseconds: timed_response.time_taken_microseconds,
                gas_used: sequence_element
                    .request
                    .gas_used()
                    .expect("expected a gas used parameter for elements we are benchmarking"),
                response: timed_response.response,
            };

            summaries.push(summary);
        }
    }

    summaries
}

fn parse_gas_used(gas_used_hex_str: &str) -> u128 {
    let hex_str_no_prefix = gas_used_hex_str.trim_start_matches("0x");
    u128::from_str_radix(hex_str_no_prefix, 16).expect("Failed to parse hexadecimal string")
}

fn compute_gas_per_second(gas_used: u128, time_ms: u128) -> u128 {
    if time_ms == 0 {
        return 0;
    }
    gas_used.saturating_mul(1_000_000) / time_ms
}

fn format_gas_rate(gas_per_second: u128) -> String {
    if gas_per_second >= 1_000_000_000 {
        // Convert to Ggas/s
        let rate = (gas_per_second as f64) / 1_000_000_000_f64;
        format!("{:.2} Ggas/s", rate)
    } else if gas_per_second >= 1_000_000 {
        // Convert to Mgas/s
        let rate = (gas_per_second as f64) / 1_000_000_f64;
        format!("{:.2} Mgas/s", rate)
    } else {
        // Just "gas/s"
        format!("{} gas/s", gas_per_second)
    }
}

// TODO: check stdlib for a possible replacement
fn natural_lexical_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    let mut a_parts = a.split(|c: char| !c.is_numeric());
    let mut b_parts = b.split(|c: char| !c.is_numeric());

    loop {
        match (a_parts.next(), b_parts.next()) {
            (Some(a_part), Some(b_part)) => {
                if let (Ok(a_num), Ok(b_num)) = (a_part.parse::<u64>(), b_part.parse::<u64>()) {
                    // If both parts are numbers, compare them numerically
                    match a_num.cmp(&b_num) {
                        std::cmp::Ordering::Equal => continue,
                        other => return other,
                    }
                } else {
                    // If not both numbers, compare lexicographically
                    match a_part.cmp(b_part) {
                        std::cmp::Ordering::Equal => continue,
                        other => return other,
                    }
                }
            }
            (None, None) => return std::cmp::Ordering::Equal,
            (None, Some(_)) => return std::cmp::Ordering::Less,
            (Some(_), None) => return std::cmp::Ordering::Greater,
        }
    }
}
