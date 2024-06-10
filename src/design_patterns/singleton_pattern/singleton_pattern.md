# 单例模式
单例模式，某种程度上可以看作是一个全局变量，在 Rust 也是以这种方式实现的，但是在 rust 中全局变量（常亮）有它的特别之处，比如全局变量或者和 const 常量需要在编译期间就能够确定该值，这就有点像 C++ 中的 constexpress ，这样的规定给 rust 全局变量带来了一些局限性，比如无法在运行期间初始化，下面这段代码中 String::from不可能在编译期间就知道结果，因此该全局变量无法在编译期间就确定值，这回导致编译错误。
``` rust
use std::sync::Mutex;
static NAMES: Mutex<String> = Mutex::new(String::from("Sunface, Jack, Allen"));

fn main() {
    let v = NAMES.lock().unwrap();
    println!("{}",v);
}
```
有许多方式可以实现运行时初始化全局静态变量
1. 使用lazy_static!，这个是一个第三方库，可以实现全局变量的运行时初始化。
2. Box::leak，它可以将一个变量从内存中leak
3. 使用cell::OnceCell 和 sync::OnceLock 