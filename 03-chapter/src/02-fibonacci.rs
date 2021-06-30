fn main() {
    let n = 8;

    println!("The {}th fibonacci number is: {}", n, fibonacci(n));
}

// With recursion, really bad code because of the exponential complexity
// fn fibonacci(x: u32) -> u32 {
//     match x {
//         1 | 2 => 1,
//         3 => 2,
//         _ => fibonacci(x - 1) + fibonacci(x - 2),
//     }
// }

// Using for loop, linear time
fn fibonacci(x: u32) -> u32 {
    match x {
        1 | 2 => 1,
        _ => {
            let mut i = 1;
            let mut j = 1;
            let mut counter = 0;

            for k in 3..x + 1 {
                counter = i + j;

                if k % 2 == 0 {
                    i = counter;
                } else {
                    j = counter;
                }
            }

            counter
        }
    }
}
