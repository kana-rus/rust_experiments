fn correct_now() -> String {
    let output_bytes = std::process::Command::new("/usr/bin/date")
        .env("LANG", "en_US")
        .arg("+'%a, %d %b %Y %H:%M:%S GMT'")
        .arg("-u")
        .output().unwrap()
        .stdout;
    String::from_utf8(output_bytes).unwrap()
}

fn main() {
    for wait in [
        4,
        12,
        33,
        15,
        8,
        23,
        42,
        1,
        36,
        29,
    ] {
        std::thread::sleep(std::time::Duration::from_secs(wait));
        println!("[now:IMF-fixdate] {}", correct_now());
    }

    println!("\nprocess finished.");
}
