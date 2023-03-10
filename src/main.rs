fn rate(line: String) -> (i32, Vec<String>) {
    let mut score = 0;
    let mut issues = Vec::new();

    for c in line.chars() {
        match c {
            'a' | 's' | 'd' | 'f' | 'g' => score += 1,
            'h' | 'j' | 'k' | 'l' => score += 1,
            _ => {
                score -= 1;
                issues.push(format!("Bad keymash character: '{}'", c));
            }
        }
    }

    let mut repeats = 0;
    let mut last_char = ' ';
    let mut last_repeat = ' ';

    for c in line.chars() {
        if c == last_char {
            repeats += 1;
            if c != last_repeat {
                issues.push(format!("Repeated character: '{}'", c));
            }
            last_repeat = c;
        }

        last_char = c;
    }

    score -= repeats;

    let negative_modifier: i32;
    let line_len = line.len() as i32;

    // Ideal keymash length is 12 characters, a grace of 1 character is given
    // Don't question it, I don't make the rules.
    if line_len > 13 {
        negative_modifier = line_len - 12;
        issues.push(format!("Keymash too long: {} characters", line_len));
    } else if line_len < 11 {
        negative_modifier = 12 - line_len;
        issues.push(format!("Keymash too short: {} characters", line_len));
    } else {
        negative_modifier = 0;
    }

    score /= (negative_modifier / 3) + 1;

    if score > 12 {
        score = 12;
    }

    (score, issues)
}

fn main() {
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        line.pop();

        let (score, issues) = rate(line);

        let score_percentage = ((score as f32 / 12.0) * 100.0) as i32;

        println!("Keymash score: {}%", score_percentage);
        for issue in issues {
            println!("  - {}", issue);
        }
        println!();
    }
}
