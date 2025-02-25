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
        let target = format!("target: {}", i.target);
        let version = i
            .version
            .map_or("".to_string(), |i| format!("version: {}", i));
        println!("{name} {target} {version}");
    }
}
