// fn main() {
//     // println!("Hello, world!");


//     let a = "hoge";
//     // let mut a = "fuga";
//     println!("{}", a);
// }


// fn main() {
//   let a = 10;
//   let aref1 = &a;
//   let aref2 = &a;
//     println!("{} / {} / {}", a, aref1, aref2);
// }

// fn main() {
//   // let mut a = 10;
//   // let _a_ref1 = &a;
//   // let _a_mut_ref1 = &mut a;
//   // let a_mut_ref2 = &mut a;
//   // *a_mut_ref2 = 20;
//   // println!("{}", a_mut_ref2);

//   let mut a = 10;           // mutable object
//   let a_ref1 = &a;          // reference
//   let a_mut_ref1 = &mut a;  // mutable reference
//   let a_mut_ref2 = &mut a;  // この時点で a_ref1, a_mut_ref1 は存在しない
//   // println!("{}", a_ref1);   // borrow check!! - Error!
//   println!("{}", a_mut_ref1);
// }

// NOTE: loopの利用
// fn main() {
//     let mut counter = 0;
//     let result = loop {
//         counter += 1;
//         if counter == 10 {
//             break counter * 2;
//         }
//     };
//     println!("The result is {}", result);
// }

// NOTE: 構造体
// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn main() {
//     let mut user1 = User {
//         email: String::from("hoge@example.com"),
//         username: String::from("someusername123"),
//         active: true,
//         sign_in_count: 1,
//     };
//     user1.email = String::from("fuga@example.com");

//     println!("email: {}", user1.email)
// }

// NOTE: タプル構造体
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let black = Color(0, 0, 0);
//     let Point(x, y, z) = Point(0, 0, 0);

//     println!("{} {} {}", black.0, black.1, black.2);
//     println!("{} {} {}", x, y, z);
// }

// NOTE: メソッド
// struct Rectangle {
//     width: u32,
//     height: u32
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle { width: 30, height: 50 };
//     println!("The area of the rectangle is {} square pixels", rect1.area());
// }

// NOTE: 列挙型

// enum IpAddrKind {
//     V4,
//     V6,
// }

// fn main() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;

//     // println!("{}", four);
//     // println!("{}", six);
// }

// NOTE: ジェネリクス
// fn add<T>(a: T, b: T) -> T {
//     a + b
// }

// struct Point<T> {x: T, y: T}

// impl<T> Point<T> {
//     fn xy(self) -> (T, T) {
//         (self.x, self.y)
//     }
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
//     a + b
// }

// fn main() {
//     let calc = add(1, 3);
//     println!("calc: {}", calc);
// }

// NOTE: match式
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// fn main() {
//     let value = value_in_cents(Coin::Nickel);
//     println!("value: {}", value);
// }

// NOTE: マクロ

// fn main() {
//     // panic!("hoge!")
//     unimplemented!("fuga!")
// }

// NOTE: イテレータ
fn main() {
    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];
    let iter = a1.iter().zip(a2.iter());
    // let iter: Vec<integer> = a1.iter().zip(a2.iter()).collect();

    for elem in iter {
        println!("elem.0: {}", elem.0);
        println!("elem.1: {}", elem.1);
    }

    // let iter2 = a1.iter().map(|x| 2 * x);

    // for elem in iter2 {
    //     println!("elm: {}", elem);
    // }
}
