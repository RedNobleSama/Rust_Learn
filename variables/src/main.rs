const MAX_POINTS: u32 = 100000; // 声明常量

fn main() {
    println!("Hello, world!");

    let  x: u32 = 5;
    let x = x + 1;
    let x = x * 2;
    println!("the value of x is {}", x);

    //Shadowing,可以使用相同的名字声明新的变量，新的变量就会shadow(隐藏)之前声明的变量

    // 复合类型
    // 元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{}, {}, {}", tup.0, tup.1, tup.2)
    // 数组,数组没有vector灵活.vector和数组类似，长度可以改变，不确定使用数组还是vector，应该使用vector
    let a: [i32; 5] = [1,2,3,4,5];
    let b =  [4; 5];
    let first = a[0];
    let second = b[1];

}
