fn main() {
    let x: (i32, i64, char) = (500, 23, 'a');

    // let (a, b, c) = x;

    println!("zz{} {}", x.0, x.1);

    let arr: [i32; 3] = [12, 33, 23];

    println!("{}, {}, {}", arr[0], arr[1], arr[2]);

    printx();
}


fn printx() {
    let x: (i32, char) = (99, '😁');

    println!("Pintx tuple int:{}, emoji:{}", x.0, x.1);
}