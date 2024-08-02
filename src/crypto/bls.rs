use num::Complex;

fn test_bls_sign() {
    let a = 10;
    let b: u16 = 100;

    let c = Complex { re: 2.1, im: -1.2 };
    let d = Complex::new(11.1, 22.2);

    let result = c + d;

    println!("{} + {}i", result.re, result.im);

    match b.try_into() {
        Ok(result) => {
            if a < result {
                println!("Ten less")
            }
        }
        Err(e) => println!("error: {}", e),
    };
}
