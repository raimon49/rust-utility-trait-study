struct Application {
    name: String,
    nicknames: Vec<String>
}

impl Drop for Application {
    // 値がヒープから解放されるタイミングでDropトレイトのdrop()が呼び出される
    // C++のデストラクタやJavaのファイナライザに似ている
    // dropメソッドは暗黙的呼び出ししか許可されず、自分で呼び出そうとするとエラーになる
    // ある型がDropトレイトを実装しているなら、Copyトレイトは実装できない
    fn drop(&mut self) {
        print!("Dropping {}", self.name);
        if !self.nicknames.is_empty() {
            // drop()で呼ばれたタイミングでは、まだ値が入っているため利用できる
            print!(" (AKA {})", self.nicknames.join(", "));
        }

        println!("");
    }
}

// ?SizedはSizedかもしれない型（questionably sized）を表現する
// デフォルトではすべての型はSizedとしてマーカートレイトされる
struct S<T: ?Sized> {
    b: Box<T>
}

struct Selector<T> {
    // 型 `Selector` で利用できる要素
    elements: Vec<T>,
    // `elements` 内の「現在使用している」要素を示す。
    // `Selector` は、現在使用している要素へのポインタとして機能する。
    current: usize
}

use std::ops::{Deref, DerefMut};

impl<T> Deref for Selector<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.elements[self.current]
    }
}

impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.elements[self.current]
    }
}

fn show_it(thing: &str) {
    println!("{}", thing);
}

use std::fmt::Display;
fn show_it_generic<T: Display>(thing: T) {
    println!("{}", thing);
}

use std::net::Ipv4Addr;
fn ping<A>(address: A)
    where A: Into<Ipv4Addr>
{
    let _ipv4_address = address.into();
}

fn main() {
    let mut _a = Application { name: "Zeus".to_string(),
                               nicknames: vec!["cloud collector".to_string(),
                                               "king of the gods".to_string()] };
    println!("before assignment");
    _a = Application { name: "Hera".to_string(), nicknames: vec![] };
    println!("at end of block");

    let _s: S<str>; // Box<str>はunsizedなので_sははunsized
    let _i: S<i32>; // Box<i32>はsizedなので、_iはsized

    let mut s = Selector { elements: vec!['x', 'y', 'z'],
                           current: 2 };
    // SelectorはDerefを実装しているので*演算子を使って現在使用している要素を参照できる
    assert_eq!(*s, 'z');

    // 'z'がアルファベットかどうかをchar型のメソッドをSelectorに対して呼び出し、参照解決変換が呼ばれチェックできる
    assert!(s.is_alphabetic());

    let current_as_str = Selector { elements: vec!["good", "bad", "ugly"],
                                    current: 2 };
    show_it(&current_as_str); // Rustコンパイラは引数の型が&Selector<&str>であり、Deref<Target=str>の実装を見つけて関数呼び出しをshow_it(s.deref())と書き換えてくれる

    show_it_generic(&current_as_str as &str); // as演算子で明示的に型変換して呼び出さないと「`Selector<&str>` doesn't implement `std::fmt::Display`」コンパイルエラーとなる

    // Iteratorトレイトのpartitionメソッドは各コレクション型の実装するDefaultトレイトで空のコレクションを作って2つに分割する
    use std::collections::HashSet;
    let squares = [4, 9, 16, 25, 36, 49, 64];
    let (powers_of_two, impure): (HashSet<i32>, HashSet<i32>) = squares.iter().partition(|&n| n & (n-1) == 0);
    assert_eq!(powers_of_two.len(), 3);
    assert_eq!(impure.len(), 4);

    // StringもDefaultとExtend<char>を実装しているため、partition() + クロージャで次のように分割できる
    let (upper, lower): (String, String) = "Great Teacher Onizuka".chars().partition(|&c| c.is_uppercase());
    assert_eq!(upper, "GTO");
    assert_eq!(lower, "reat eacher nizuka");

    // fn open<P: AsRef<Path>>(path: P) -> Result<File> が要求するのはAsRefを実装した型であるため、
    // Stringやstrも引数に取ることができる
    let _dot_emacs = std::fs::File::open("/home/jimb/.emacs");

    // ping()はInto<Ipv4Addr>を実装していて.into()が呼べることを期待している
    ping(Ipv4Addr::new(23, 21, 68, 141));
    ping([66, 146, 219, 98]);
    ping(0xd076eb94_u32);

    // Fromトレイトはある型から別の型のインスタンスを生成する汎用コンストラクタとして機能する
    let addr1 = Ipv4Addr::from([208, 118, 235, 148]);
    let addr2 = Ipv4Addr::from(0xd076eb94_u32);
    assert_eq!(addr1, addr2);

    // StringのInto<Vec<u8>>の実装は、Stringのヒープバッファを取り、そのままベクタの要素バッファとして返す
    // 所有権が移動しているため変数textはもう使えない点に注意
    let text = "Beautiful Soup".to_string();
    let bytes: Vec<u8> = text.into();
    println!("{} bytes", bytes.len());
    for (_i, b) in bytes.iter().enumerate() {
        print!("{:02.x} ", b); // bytesの中身を16進数文字コードで表示:w

    }
    println!();
    println!("{}", String::from_utf8(bytes).unwrap()); // bytesをStringに戻して表示
}
