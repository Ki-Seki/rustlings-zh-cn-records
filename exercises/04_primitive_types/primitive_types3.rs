fn main() {
    // TODO: 创建一个名为`a`，至少包含100个元素的数组。
    let a = [0; 101];

    if a.len() >= 100 {
        println!("哇，很大的数组欸!");
        println!("{}", a.len())
    } else {
        println!("咔嚓，嘎嘣脆，鸡肉味");
        panic!("数组不够大, 我需要更多的元素!");
    }
}
