fn concat_string(prefix: &str,s: &String) -> String {
    let mut res = String::new();
    res.push_str(prefix);
    res.push_str(s.as_str());
    return res;
}

struct Foo {
    s: String,
}

fn main() {
    let x = String::from("hello world");
    let y = x;
    // error: value borrowed here after move
    // println!("x: ({})", x);
    println!("y: ({})", y);  // ok

    let x1 =  String::from("hello world 1");
    let rx1 = &x1;
    let rx2 = &rx1;
    println!("reference of x1 (&String): ({})", rx1); // ok
    println!("reference of reference of x1 (&&String): ({})", rx2); // ok

    // 不可变引用绑定到变量后，变量为引用类型，引用与右边相同的内存
    // 右边的引用不会发生shadow copy，仍然可以访问，且引用与之前相同的内存
    let x3 = rx1;
    println!("binding of reference of x1 (&String): ({})", x3); // ok
    println!("access reference of x1 again: ({})", rx1); // ok

    let x4 = concat_string("hey", rx1);
    println!("x4: ({})", x4);
    println!("access rx1 again: ({})", rx1);

    let f = Foo {
        s: String::from("hello"),
    };
    // let x = f.s;
    // error: value borrowed here after move
    // println!("f.s: ({})", f.s);

    let r = &f.s;
    println!("f.s: ({})", f.s); // ok
}
