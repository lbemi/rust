use std::fs::File;
use std::io::{self,ErrorKind, Read};


fn main() {
    // panic!("pacnic");
    let v = vec![1,2,3];
    // v[10];

    // Result();

    // result2();
    // unwarp();
    // expect();
    let f = read_username_from_file();
}

fn result() {
    let f = File::open("hello.txt");
   let f =  match f {
        Ok(file) => file,
        Err(error) => {
            panic!("error opening file {:?}",error);
        },

    };
}

fn result2() {
    let f = File::open("hello.txt");
   let f =  match f {
        Ok(file) => {
            println!("文件已存在!");
            file
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => {
                    println!("create file : hello.txt success.");
                    fc
                },
                Err(error) => panic!("Error creating file: {:?}", error),
            } ,
            oe => panic!("Error opening file : {:?}",oe),
        }
    };
}


// 简写
fn result3() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Error creating file : {:?}", error);
            })
        } else {
            panic!("Error opening file: {:?}", error);
        }
    });
}

fn unwarp() {
    // unwrap将ok的值返回;简化macth
    let f = File::open("hello.txt").unwrap();
}

fn expect() {
    // 可以定义自定义错误
    let f = File::open("hello.txt").expect("无法打开文件");
}

fn read_username_from_file() -> Result<String,io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) =>file,
        Err(e) => return Err(e)
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
//简化-传播错误?
fn read_username_from_file2() -> Result<String,io::Error> {
    let mut f = File::open("hello.txt")?;
    // let mut f = match f {
    //     Ok(file) =>file,
    //     Err(e) => return Err(e)
    // };
    let mut s = String::new();
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
    f.read_to_string(&mut s)?;

    Ok(s)
}

//简化-传播错误? + 链式调用
fn read_username_from_file3() -> Result<String,io::Error> {
    let mut s = String::new();
     File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}