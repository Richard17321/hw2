// use std::fmt;
// fn cal_sum(x: &[u32]) -> Option<u32> {
//     match x {
//         None => None,
//         Some(a) => {
//             a.iter().sum()
//         }
//     }
// }

// fn cal_sum(x: &[u32]) -> Option<u32> {
//     let sum: u32 =  x.iter().sum();
//     // let b = Some(sum);
//     match sum {
//         None => None,
//         Some(a) => println!("Sum is: {}", a),
//     }
// }

// fn cal_sum(x: &[u32]) -> Option<u32> {
//     let sum: u32 =  x.iter().sum();
//     let b = Some(sum);
//     match b {
//         Some(a) => Some(a),
//         None => None,
//     }
// }

// fn cal_sum(x: &[u32]) -> Option<u32> {
//     let sum: u32 =  x.iter().sum();
//     if sum > x[1] {
//         return Some(sum);
//     }
//     None
// }

// fn cal_sum(x: &[u32]) -> Option<u32> {
//     // let total = ;
//     // let result = Some(total);
//     // println!("result is: {:?}", result);
//
//     match Some(x.iter().sum()) {
//         Some(a) => Some(a),
//         None => None
//     }
// }
//
// fn cal_sum(x: &[u32]) -> Option<u32> {
//     let mut total = Some(0);
//     for item in x.iter() {
//         match total.as_mut() {
//             Some(total) => Some(total += item),
//             None => None,
//         };
//     }
//     total
// }



// fn cal_sum(x: &[u32]) -> Option<u32> {
//     let mut total = 0;
//     for item in x.iter() {
//         match Some(total + item) {
//             Some(a) => {
//                 total = a;
//             },
//             None => {
//                 None;
//             }
//         };
//     }
//     Some(total)
// }

// fn cal_sum(x: &[u32]) -> Option<u32> {
//     let mut total = 0;
//     for item in x.iter() {
//         match Some(total + item) {
//             Some(a) => {
//                total = a;
//             },
//             None => { break; }
//         };
//     }
//     Some(total)
// }
//


use std::f32::consts::E;

fn cal_sum(x: &[u32]) -> Option<u32> {
    match x.iter().sum() {
        Ok(val) => Some(val),
        Err(_) => None,
    }
}




fn main() {
    let numbers = [1, 2, 3];
    // let numbers = [4294967294, 1, 2];
    let total = cal_sum(&numbers);
    match total {
        Some(a) => println!("Sum is: {}", a),
        None => println!("Overflow!")
    }
}

