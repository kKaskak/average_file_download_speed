use tokio::process::Command;
use std::str;
use std::time::Duration;
use tokio::time::sleep;
use std::env;
use std::fs;
use regex::Regex;

const NUM_RUNS: usize = 3;

#[tokio::main]
async fn main() {
    // Get the home directory
    let home_dir = env::var("HOME").expect("Failed to get HOME directory");
    let curl_format_file_path = format!("{}/curl-format.txt", home_dir);

    // Read the content of the curl-format.txt file
    let curl_format = fs::read_to_string(&curl_format_file_path)
        .expect("Failed to read curl-format.txt file");

    let mut total_time: f64 = 0.0;

    for i in 0..NUM_RUNS {
        let output = Command::new("curl")
            .arg("-w")
            .arg(&curl_format)
            .arg("-o")
            .arg("/dev/null")
            .arg("-s")
            .arg("YOUR_URL_HERE")
            .output()
            .await
            .expect("Failed to execute curl command");

        let stdout_str = match str::from_utf8(&output.stdout) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to convert stdout to string: {}", e);
                continue;
            }
        };

        // Extract time_total from the output
        let re = Regex::new(r"time_total:\s*([\d\.]+)").expect("Failed to compile regex");
        if let Some(captures) = re.captures(stdout_str) {
            if let Some(time_str) = captures.get(1) {
                let time_value: f64 = time_str.as_str().parse().expect("Failed to parse time_total value");
                total_time += time_value;
            }
        }

        println!("Curl output:\n{}", stdout_str);

        // Sleep for a short duration to avoid spamming the server
        sleep(Duration::from_millis(100)).await;
    }

    let average_time = total_time / NUM_RUNS as f64;
    println!("Average total time over {} runs: {:.3} seconds", NUM_RUNS, average_time);
}
