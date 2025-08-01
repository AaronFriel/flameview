fn main() {
    let mut sum = 0u64;
    for i in 0..50_000_000u64 {
        sum = sum.wrapping_add(std::hint::black_box(i));
    }
    std::hint::black_box(sum);
}
