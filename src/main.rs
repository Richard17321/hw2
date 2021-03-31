fn cal_sum(x: &[u32]) -> Option<u32> {
    let mut total= 0_u32;
    for item in x.iter() {
        match total.checked_add(*item) {
            Some(a) => {
               total = a;
            },
            None => { return None; }
        };
    }
    Some(total)
}



fn main() {
    let numbers = [1, 2, 3, 4];
    // let numbers = [4294967294, 1, 2];
    let total = cal_sum(&numbers);
    match total {
        Some(a) => println!("Sum is: {}", a),
        None => println!("Overflow!")
    }
}

