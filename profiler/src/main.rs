mod bench_summary;
mod docker;
mod engine_api;
mod kute;

use bench_summary::{BenchEngineAPIRequestSummary, BenchInput};
use cli_table::{print_stdout, Cell, Style, Table};
use indicatif::ProgressIterator;
use kute::JwtClient;
use std::fs;

#[tokio::main]
async fn main() {
    // Parse Engine API requests that we want to benchmark
    let mut bench_inputs = read_gas_limit_files("tests/GasLimit").unwrap();
    // Sort the inputs by name, so they are printed in natural lexicographical order
    bench_inputs.sort_by(|a, b| natural_lexical_cmp(&a.name, &b.name));

    let mut rows = Vec::new();
    let mut header = vec!["Name".cell().bold(true), "Description".cell().bold(true)];
    // push the client names to the header
    for client in get_clients() {
        header.push(client.cell().bold(true));
    }

    for bench_input in bench_inputs.into_iter().progress() {
        let mut column = Vec::new();
        column.push(bench_input.name.clone().cell());
        column.push(bench_input.description.clone().cell());

        for client in get_clients() {
            let dc = docker::DockerCompose::new(&format!("{}.yml", client));
            dc.up().unwrap();

            // TODO: Add a health check instead of a manual delay.
            tokio::time::sleep(std::time::Duration::from_secs(15)).await;

            let summary = benchmark_engine_api_request(&bench_input).await;

            // For each summary, we have a table column
            for sum in summary {
                let gas_used_u128 = parse_gas_used(&sum.gas_used);
                let gas_per_second =
                    compute_gas_per_second(gas_used_u128, sum.time_taken_microseconds);

                column.push(format_gas_rate(gas_per_second).cell());
            }

            dc.down().unwrap();
        }

        rows.push(column);
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
