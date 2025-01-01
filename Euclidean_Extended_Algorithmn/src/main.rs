


fn main() {
    // Numerical inputs
    let foo = 126;
    let goo = 35;
    let mut s_0 = 1;
    let mut s_1 = 0;
    let mut t_0 = 0;
    let mut t_1 = 1;


    let mut r_0 = foo;
    let mut r_1 = goo;
    let mut index = 1;

    while r_1 != 0 {
        println!("Step {}: ", index);


        let quotient = r_0 / r_1;

        let remainder = r_0 - quotient * r_1;
        r_0 = r_1;
        r_1 = remainder;

        let new_s_0 = s_0 - quotient * s_1;
        s_0 = s_1;
        s_1 = new_s_0;

        let new_t_0 = t_0 - quotient * t_1;
        t_0 = t_1;
        t_1 = new_t_0;


        println!("r_0: {}, r_1: {}", r_0, r_1);
        println!("s_0: {}, s_1: {}", s_0, s_1);
        println!("t_0: {}, t_1: {}", t_0, t_1);


        index += 1;
    }

    println!("\nFinal results:");
    println!("gcd: {}", r_0);
    println!("s: {}", s_0);
    println!("t: {}", t_0);
}

/* what i learned
    - i suck at formatting ,have to get more consistent
    - write down notes so translating from psed code is easier
    - need to slow down and pay attention sub indicies meaning
 */