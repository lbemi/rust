```bash
# 使用cargo创建项目
➜  rust-learn cargo new hello-cargo
     Created binary (application) `hello-cargo` package
➜  rust-learn cd hello-cargo      

# 使用cargo编译项目
➜  hello-cargo git:(master) ✗ cargo build          
   Compiling hello-cargo v0.1.0 (/Users/lei/tmp/rust-learn/hello-cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 1.39s

# 使用cargo运行项目
➜  hello-cargo git:(master) ✗ cargo run  
   Compiling hello-cargo v0.1.0 (/Users/lei/tmp/rust-learn/hello-cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 1.37s
     Running `target/debug/hello-cargo`
Hello, world!
➜  hello-cargo git:(master) ✗ 

# 检查
➜  hello-cargo git:(master) ✗ cargo check
    Checking hello-cargo v0.1.0 (/Users/lei/tmp/rust-learn/2-cargo/hello-cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.74s

# 为发布构建项目,编译时会进行优化
#   代码会运行的更快,编译时间长
#   生成路径target/release下

➜  hello-cargo git:(master) ✗ cargo build --release
   Compiling hello-cargo v0.1.0 (/Users/lei/tmp/rust-learn/2-cargo/hello-cargo)
    Finished release [optimized] target(s) in 0.88s
```