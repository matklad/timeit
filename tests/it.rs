#[test]
fn smoke_test() {
    let _it = timeit::timeit("loooong");
    std::thread::sleep(std::time::Duration::from_millis(199));
}
