use anyhow::Result;
use std::path::Path;

fn main() -> Result<()> {
    let mut times = vec![];

    for n in 1..25 {
        let prog = format!("target/release/p{:02}", n);
        let data = format!("data/p{:02}.txt", n);

        if !Path::new(&prog).exists() {
            continue;
        }

        println!("{}", prog);
        let start = time::OffsetDateTime::now_utc();
        std::process::Command::new("sh")
            .arg("-c")
            .arg(format!("{} < {}", prog, data))
            .status()?;

        let end = time::OffsetDateTime::now_utc();
        times.push((prog, end - start));
        println!();
    }

    println!("Times");
    println!("----------------------------------------");
    for (p, t) in &times {
        println!("{}: {:8}ms", p, t.whole_milliseconds());
    }

    let total = times.iter().map(|x| x.1.whole_milliseconds()).sum::<i128>();
    println!("\nTotal time:         {:8}ms", total);

    Ok(())
}
