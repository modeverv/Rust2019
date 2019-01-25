fn main() {
    println!("Hello, world!");
    // 変数束縛
    let x = 5;
    // パターンです
    let (xx, y) = (1, 2);
    // 型アノテーション
    let yy: i32 = 5;
    let mut yyy = 5;
    yyy = 10;
    println!("{}", x);
    print_number(y);
    println!("{}", add_one(1));
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn add_one(x: i32) -> i32 {
    x + 1
}


