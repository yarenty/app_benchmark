/// TEST APP, run:
/// ```shell
/// cargo build --example test_app -r   
/// cargo run --bin benchmark -- /opt/workspace/app_banchmark/target/release/examples/test_app   
/// ```
fn main() {
    println!("Hello world");
    for i in 1..1000000 {
        if i % 1000 == 0 {
            println!(" {}", i)
        };
    }
}
