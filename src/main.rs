fn main() {
    let handles1: Vec<_> = (0..100)
        .map(|id| {
            std::thread::spawn(move || loop {
                cargo_build::warning!("Hello, WARNING from {id}");
            })
        })
        .collect();

    handles1.into_iter().for_each(|t| {
        t.join().unwrap();
    });
}
