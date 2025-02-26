use guess_target::{get_local_target, guess_target};

fn main() {
    println!(
        "local target: {}",
        get_local_target()
            .iter()
            .map(|i| i.to_str())
            .collect::<Vec<_>>()
            .join(",")
    );

    let Some(name) = std::env::args().nth(1) else {
        println!("guess-target <name>");
        return;
    };

    for i in guess_target(&name) {
        let name = format!("name: {}", i.name);
        let rank = format!("rank: {}", i.rank);
        let target = format!("target: {}", i.target);
        let mut s = vec![name, rank, target];

        if let Some(v) = i.version {
            s.push(format!("version: {}", v));
        }

        println!("{}", s.join(" "));
    }
}
