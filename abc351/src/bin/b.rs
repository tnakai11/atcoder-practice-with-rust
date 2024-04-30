// https://atcoder.jp/contests/abc351/tasks/abc351_b
// (0,0)から(N-1,N-1)まで配列A,Bの要素を比較して不一致が出たら出力する

use proconio::input;

// proconioにChars型(Vec<char>)が定義されている
// https://docs.rs/proconio/latest/proconio/
use proconio::marker::Chars;

fn main() {
    input! {
        // 配列へのアクセスはusizeで行う
        // https://doc.rust-lang.org/std/primitive.usize.html
        n: usize,
        a: [Chars; n],
        b: [Chars; n],
    }

    // forを回す
    // https://doc.rust-jp.rs/rust-by-example-ja/flow_control/for.html
    for i in 0..n {
        for j in 0..n {
            if a[i][j] != b[i][j] {
                println!("{} {}", i + 1, j + 1);
                return;
            }
        }
    };
}
