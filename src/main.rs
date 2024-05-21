use std::{
    fs::{File, OpenOptions},
    io::{BufRead as _, BufReader, Write as _},
};

use chrono::{DateTime, Duration, Utc};
use eyre::{Context as _, Result};

fn main() -> Result<()> {
    // Open the file in append mode
    std::fs::remove_file("times.txt").unwrap_or(());

    std::fs::write("times.txt", "")?;

    let mut out = OpenOptions::new()
        .append(true)
        .open("times.txt")
        .wrap_err("Unable to open file times.txt")?;

    // Open the file in read-only mode
    let input = File::open("log.txt").wrap_err("Unable to open file log.txt")?;

    // Create a buffered reader
    let reader = BufReader::new(input);

    // total_secs and total_values are used to calculate the average
    let mut total_secs = 0;
    let mut total_blocks = 0;

    let last = DateTime::parse_from_rfc3339("2024-05-21T13:54:10Z")
        .expect("Failed to parse datetime")
        .with_timezone(&Utc);

    // Read the file line by line
    let mut previous_datetime = None;
    for line in reader.lines() {
        let line = line?;
        let datetime_str = &line[0..20];

        let update_tip = &line[21..30];
        if update_tip != "UpdateTip" {
            continue;
        }

        let datetime: DateTime<Utc> = DateTime::parse_from_rfc3339(datetime_str)
            .expect("Failed to parse datetime")
            .with_timezone(&Utc);

        if datetime < last {
            continue;
        }

        if let Some(previous_datetime) = previous_datetime {
            let duration: Duration = datetime - previous_datetime;
            let num_secs = duration.num_seconds();
            let line = format!("{}: {}s", datetime_str, num_secs);

            total_secs += num_secs;
            total_blocks += 1;

            writeln!(out, "{}", line)?;
        }

        previous_datetime = Some(datetime);
    }

    let avg_secs = total_secs as f32 / total_blocks as f32;
    println!("Total Seconds: {total_secs}");
    println!("Total Blocks: {total_blocks}");
    println!("Average Seconds Per Block: {avg_secs}s");

    writeln!(out, "Average: {}s", avg_secs)?;

    Ok(())
}
