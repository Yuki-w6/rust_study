fn main () {
    let mut n_nonzero = 0;

    for i in 1..10000 {
        let ptr = i as *const u8;
        let byte_at_addr = unsafe { *ptr};

        fi byte_at_addr != 0 {
            n_nonzero += 1;
        }
    }

    println!("non-zero bytes in memory: {}", n_nonzero);
}