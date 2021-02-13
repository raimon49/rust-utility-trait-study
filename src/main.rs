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
}
