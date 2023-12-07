pub fn test_function()->u32{ 0}

pub fn testspeed(){
    println!("start");
    let now = Instant::now();
    for _ in 0..10000000000u128 {
        std::hint::black_box(test_function());
    }
    let elapsed_time = now.elapsed();
    println!("end");
    println!("Running it took {:?} seconds.", elapsed_time);
}