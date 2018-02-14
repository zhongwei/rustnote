#[cfg(target_os = "macos")]
fn are_you_on_macos() {
    println!("You are running macos!")
}

#[cfg(not(target_os = "macos"))]
fn are_you_on_macos() {
    println!("You are *not* running macos!")
}

fn main() {
    are_you_on_macos();

    println!("Are you sure?");

    if cfg!(target_os = "macos") {
        println!("Yes. It's definitely macos!");
    } else {
        println!("Yes. It's definitely *not* macos!");
    }
}
