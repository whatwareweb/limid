#[cfg(not(feature = "wasm"))]
fn main() {
    println!("Hello, world!");
    let _handle = limid::tone();
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
