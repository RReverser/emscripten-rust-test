fn main() {
    std::thread::spawn(|| {
        println!("Hello from another thread");
    })
    .join()
    .unwrap();
}
