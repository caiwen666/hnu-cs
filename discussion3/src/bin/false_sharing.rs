use discussion3::test_false_sharing;

fn main() {
    let timer = std::time::Instant::now();
    test_false_sharing();
    println!("time = {:?}", timer.elapsed());
}
