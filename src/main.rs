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
    println!("インクリメント関数{}", add_one(1));
    // 関数ポインタ
    let f: fn(i32) -> i32 = add_one;
    println!("インクリメント関数(関数ポインタ) {}", f(1));
    // プリミティブ
    // bool
    let px = true;
    let py:bool = false;
    let pc:char = 'x';
    /*
i8
i16
i32
i64
u8
u16
u32
u64
isize
usize
f32
f64
     */
    // 配列
    let mut pa = [0, 1, 2, 3, 4];
    // let pa = [0; 20];
    println!("{}", pa.len());
    // スライス
    let middle = &pa[1..4]; // [1,2,3]
    
    // if
    if x == 5 {
        println!("x は 5 です!");
    } else if x == 6 {
        println!("x は 6 です!");
    } else {
        println!("x は 5 でも 6 でもありません :(");
    }

    let ify = if x == 5 {
        10
    } else {
        15
    }; // y: i32
    let ifyy = if x == 5 { 10 } else { 15 }; // y: i32

    // for
    for x in 0..10 {
        println!("{}", x);
    }
    for (i,j) in (5..10).enumerate() {
        println!("i = {} and j = {}", i,j);
    }
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 { continue 'outer; } // x のループを継続
            if y % 2 == 0 { continue 'inner; } // y のループを継続
            println!("x: {}, y: {}", x, y);
        }
    }
}

/// # markdown comment!!!
///
/// ```
/// usage : print_number(13);
/// ````
///
fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn add_one(x: i32) -> i32 {
    //returnはよろしくないスタイルとされている
    // return x + 1;
    x + 1

}


