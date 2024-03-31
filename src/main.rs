fn main() {
    let data = "initial contents";
    let s0 = data.to_string();
    println!("{}", s0);

    let s1 = "No.1".to_string();
    println!("{}", s1);

    let mut s2 = String::new();
    s2.push_str("No.2");
    s2.push('!');
    println!("{}", s2);

    let mut s3 = String::from("No.3");
    s3.push_str("!");
    println!("{}", s3);

    let s4 = "No.4!".to_string() + &s3 + &s2 + &s1; // 后边的参数必须为借用，是对第一个String参数的所有全转移到s4
    println!("{}", s4);
    s2.push_str("change this s2 part!");
    println!("{}", s4); // s4 借用了s2，但是s2变化后，s4却没有变化。原因是，s4的拼接是对原s2的引用字符串拼接，对之后的不管。

    let s5 = format!("No.5-{s1}-{s2}-{s3}-{s4}"); // 这里使用format!宏，不获取任何参数的所有权，都是借用。
    println!("{}", s5);

    // Rust 的字符串不支持索引。这是使用 UTF-8 编码 “Здравствуйте” 所需要的字节数，这是因为每个 Unicode 标量值需要两个字节存储。
    let hello = String::from("Здравствуйте");
    println!("len of '{}' is {}", hello, hello.len());
    // 最后一个 Rust 不允许使用索引获取 String 字符的原因是，索引操作预期总是需要常数时间（O(1)）。但是对于 String 不可能保证这样的性能，因为 Rust 必须从开头到索引位置遍历来确定有多少有效的字符。

    // 对于单独的 Unicode 标量值使用 chars 方法。
    for c in "Зд".chars() {
        println!("{c}");
    }

    // bytes 方法返回每一个原始字节
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
