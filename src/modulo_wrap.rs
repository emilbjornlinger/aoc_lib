pub fn modulo_wrap(num: i32, base: u32) -> i32 {
    let mut res = num;
    if num < 0 {
        while res < 0 {
            res += base as i32;
        }
    } else if num > (base - 1) as i32 {
        while num > (base - 1) as i32 {
            res -= base as i32;
        }
    }

    res
}
