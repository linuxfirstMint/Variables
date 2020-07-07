const MAX_POINTS: u32 = 100_000;
// 定数の宣言はconst
// 型注釈を必ず行う
// アンダーバーで区切り 100,000 → 100_000

fn main() {
    // 不変変数
    let x = 5;
    println!("The value of x is {}", x);
    // 可変変数
    let mut y = 5;
    println!("The value of y is {}", y);
    y = 6;
    println!("The value of y is {}", y);

    println!("The value of MAX_POINTS is {}", MAX_POINTS);

    // シャドーイング
    let x = x + 1;
    let x = x * 2;
    // letをつけると再代入できる
    // x = x + 1;
    // 不変変数への再代入としてコンパイルエラー
    println!("The value of x is {}", x);

    // 変数名を使いまわせる(spaces)
    let spaces = "    ";
    println!("The value of spaces is {}", spaces);
    // spacesは文字列型

    let spaces = spaces.len();
    println!("The value of spaces is {}", spaces);
    // spacesは数値型

    // mutでは型の不一致でコンパイルエラー
    // 変数の型を変えることはできない
    //let mut spaces = "    ";
    //println!("The value of spaces is {}", spaces);
    // spacesは文字列型

    //spaces = spaces.len();
    //println!("The value of spaces is {}", spaces);
    // spacesは数値型だが
    // コンパイラは文字列型と推測
}
