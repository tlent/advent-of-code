use anyhow::{anyhow, ensure, Context, Result};
use reqwest::blocking::Client;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const LATEST_YEAR: u32 = 2023;
const EARLIEST_YEAR: u32 = 2015;

const ADVENT_OF_CODE_URL: &str = "https://adventofcode.com";

const COOKIE_PATH: &str = "project_generator/cookie";

const TEMPLATES_PATH: &str = "project_generator/templates";
const CARGO_TEMPLATE: &str = "Cargo.toml.template";
const MAIN_TEMPLATE: &str = "main.rs.template";
const LIB_TEMPLATE: &str = "lib.rs.template";
const BENCH_TEMPLATE: &str = "bench.rs.template";

fn main() -> Result<()> {
    let (year, day_number) = read_args()?;
    let day_str = format!("day_{:02}", day_number);
    let project_path = PathBuf::from(format!("year_{year}/{day_str}"));
    let src_path = project_path.join("src");
    fs::create_dir_all(&src_path)?;
    let pairs = [
        (CARGO_TEMPLATE, project_path.join("Cargo.toml")),
        (MAIN_TEMPLATE, src_path.join("main.rs")),
        (LIB_TEMPLATE, src_path.join("lib.rs")),
        (BENCH_TEMPLATE, src_path.join("bench.rs")),
    ];
    let templates_path = Path::new(TEMPLATES_PATH);
    for (template_name, output_path) in pairs {
        let template_path = templates_path.join(template_name);
        let template = fs::read_to_string(template_path)?;
        let rendered = template.replace("{{ day }}", &day_str);
        fs::write(output_path, rendered)?;
    }
    let url = format!("{}/{}/day/{}/input", ADVENT_OF_CODE_URL, year, day_number);
    let cookie = fs::read_to_string(COOKIE_PATH)?;
    let client = Client::new();
    let response = client.get(url).header("cookie", cookie.trim()).send()?;
    let input_path = project_path.join("input.txt");
    fs::write(input_path, response.text()?)?;
    Ok(())
}

fn read_args() -> Result<(u32, u32)> {
    let mut args = env::args().skip(1);
    let year_str = args.next().ok_or_else(|| anyhow!("No year provided"))?;
    let mut year = year_str.parse().context("Year must be a number")?;
    if year < 100 {
        year += 2000;
    }
    ensure!(
        (EARLIEST_YEAR..=LATEST_YEAR).contains(&year),
        "Year must be in the range {}..={}",
        EARLIEST_YEAR,
        LATEST_YEAR
    );
    let day_str = args.next().ok_or_else(|| anyhow!("No day provided"))?;
    let day = day_str.parse().context("Day must be a number")?;
    ensure!((1..=25).contains(&day), "Day must be between 1 and 25");
    Ok((year, day))
}
