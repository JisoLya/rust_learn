fn main() {
    /*
    cargo test默认行为:
    - 并行运行
    - 所有测试
    - 不捕获所有的输出 (println!()这样的)

    显示test之后的可用参数
    cargo test --help
    显示--之后可选的参数
    cargo test -- --help

    控制线程数量：
        cargo test -- --test-threads=1
    显示函数输出:
        cargo test -- --show-output
    按名称运行测试
        cargo test xxx测试名称(会自动进行匹配)

    加入#[ignore]的测试不会被运行
    运行被忽略的测试
    cargo test -- --ignored
     */
}

#[test]
fn test() {
    println!("Hello, world!");
}

#[test]
fn test2_() {
    println!("Hello, world!");
}

#[test]
fn test_3() {
    println!("Hello, world!");
}

#[test]
fn test_42() {}