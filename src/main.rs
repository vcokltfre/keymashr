use dotenv::dotenv;

mod discord;
mod rate;

fn main() {
    if let Some(arg) = std::env::args().nth(1) {
        if arg == "discord" {
            dotenv().ok();
            if let Err(why) = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(discord::run())
            {
                println!("Error: {:?}", why);
            }
        } else {
            println!("Unknown argument: {}", arg);
        }

        return;
    }

    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        line.pop();

        let (score, issues) = rate::rate(line);

        let score_percentage = ((score as f32 / 12.0) * 100.0) as i32;

        println!("Keymash score: {}%", score_percentage);
        for issue in issues {
            println!("  - {}", issue);
        }
        println!();
    }
}
