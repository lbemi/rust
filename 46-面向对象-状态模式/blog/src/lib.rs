
trait State {
    /*
    Box<T>除了数据被储存在堆上而不是栈上之外，box 没有性能损失。不过也没有很多额外的功能。它们多用于如下场景：

        - 当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候
        - 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候

        - 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候
    */
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}
# [test]
fn test() {
    let a = 6;
    let b = Box::new(a);
    println!("{}-{}",a,b);
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            // 初始化的时候state状态为 Draft
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    // 添加文章内容
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    //查看文章内容
    pub fn content(&self) -> &str {
        // as_ref 返回Options引用,当前状态: Published
        self.state.as_ref().unwrap().content(&self)
    }

    // 申请审批
    pub fn request_review(&mut self) {
        // self.state.take()获取state状态及其所有权(Draft)
        if let Some(s) = self.state.take() {
            // 调用s.request_review()方法进行审批,并将状态更新为 PendingReview
            self.state = Some(s.request_review())
        }
    }

    // 审批
    pub fn approve(&mut self) {
        // self.state.take()获取state状态及其所有权(PendingReview)
        if let Some(s) = self.state.take() {
            // 调用s.approve()方法进行审批,并将状态更新为 Published
            self.state = Some(s.approve())
        }
    }
}

//草稿
struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        println!("********* Draft-request_review--");
        Box::new(PendingReview {})
    }

    // 针对草稿,approve无需做什么
    fn approve(self: Box<Self>) -> Box<dyn State> {
        println!("********* Draft-approve--");
        self
    }
}

struct PendingReview {}
impl State for PendingReview {
    // approve返回 publish struct
    fn approve(self: Box<Self>) -> Box<dyn State> {
        println!("####### PendingReview-approve--");
        Box::new(Published {})
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        println!("####### PendingReview-request_review--");
        self
    }
}

struct Published {}
impl State for Published {
    fn approve(self: Box<Self>) -> Box<dyn State> {
        println!("--Published-approve--");
        self
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        println!("--Published-request_review--");
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        println!("--Published-content--");
        &post.content
    }
}
