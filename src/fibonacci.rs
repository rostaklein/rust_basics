pub fn fibonacci(n: u32) {
    let mut sum = 0;
    let mut last = 0;
    let mut curr = 1;

    for _i in 1..n {
        sum = last + curr;
        last = curr;
        curr = sum;
    }
    println!("{}", sum);
}
