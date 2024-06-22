use anyhow::Result;

use derive_more::{Add, Display, From, Into};

#[derive(PartialEq, Clone, Copy, From, Add, Into, Display)]
struct MyInt(i32);

#[derive(PartialEq, From)]
struct Point {
    x: MyInt,
    y: MyInt,
}

#[derive(Display, PartialEq, From, Add)]
enum MyEnum {
    #[display(fmt = "Int({_0})")]
    Int(i32),

    Uint(u32),
    #[display(fmt = "nothing")]
    Nothing,
}

fn main() -> Result<()> {
    let a = MyInt(1);
    let b = MyInt(2);
    let c = a + b;
    println!("{} + {} = {}", a, b, c);

    let my_int: MyInt = 10.into();
    let v = my_int + 20.into();
    let v1: i32 = v.into();

    println!("my_int: {}, v: {}, v1: {}", my_int, v, v1);

    let p = Point {
        x: MyInt(1),
        y: MyInt(2),
    };
    let q = Point::from((MyInt(1), MyInt(2)));
    println!("p == q: {}", p == q);

    let e: MyEnum = 10i32.into();
    let e2: MyEnum = 20u32.into();

    let e3 = MyEnum::Nothing;

    println!("e: {}, e2:{} ,e3: {}", e, e2, e3);

    Ok(())
}
