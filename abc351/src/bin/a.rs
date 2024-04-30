// https://atcoder.jp/contests/abc351/tasks/abc351_a
// sum(A) < sum(B) + x を満たす最小のxを求める問題
// x = max(sum(A) - sum(B) + 1, 0)

// use宣言で他のcrateの要素を新しい名前にバインドできる
// https://doc.rust-jp.rs/rust-by-example-ja/mod/use.html
use proconio::input;

fn main() {
    // proconio::inputマクロで標準入力を得る
    // https://docs.rs/proconio/latest/proconio/
    // https://doc.rust-jp.rs/book-ja/ch19-06-macros.html
    input! {
        a: [i32; 9],
        b: [i32; 8],
    }

    // IteratorのSumメソッドで和を得る
    // https://doc.rust-lang.org/std/iter/trait.Sum.html
    // https://doc.rust-jp.rs/book-ja/ch13-02-iterators.html
    let sum_a: i32 = a.iter().sum();
    let sum_b: i32 = b.iter().sum();

    // std::cmp::maxで最大値を得る
    // https://doc.rust-lang.org/std/cmp/fn.max.html
    let ans = std::cmp::max(sum_a - sum_b + 1, 0);

    // println!マクロで標準出力にテキストを出力する
    // https://doc.rust-lang.org/std/macro.println.html
    println!("{}", ans);
}
