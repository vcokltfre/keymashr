use dotenv::dotenv;

mod discord;
mod rate;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 2 {
        if args[1] == "discord" {
            println!("Running in Discord mode...");
            dotenv().ok();
            let future = discord::run();
            if let Err(why) = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(future)
            {
                println!("Error: {:?}", why);
            }
            return;
        }

        println!("Unknown argument: {}", args[1]);
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
