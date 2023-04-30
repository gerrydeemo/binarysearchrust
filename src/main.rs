use std::io;

fn binary(termval: i32) {
    let arrayofnums: [i32; 20] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    ];
    let mut term = termval;
    let mut low = 0;
    let mut high = arrayofnums.len() - 1;
    while high >= low {
        let mid = (high + low) / 2;
        if arrayofnums[mid] == term {
            println!("found {}", term);
            break;
        }
        if arrayofnums[mid] > term {
            high = mid - 1
        }
        if arrayofnums[mid] < term {
            low = mid + 1
        }
    }
}

fn linear(termval: i32) {
    let mut term = termval;
    let arrayofnums: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for i in arrayofnums {
        if i == term {
            println!("found {}", i)
        }
    }
}

fn main() {
    println!("Input your number: ");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    let num: i32 = number.trim().parse().expect("Not a number");
    binary(num);
    linear(num);
}
