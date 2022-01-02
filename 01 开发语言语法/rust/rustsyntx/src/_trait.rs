// trait 特征

// 1 定义trait
pub trait Summary{
    fn summarize(&self) -> String; 
}
// 2 为类型实现trait
pub struct NewArticle {
    pub headline: String,
    pub location: String,
}
impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{},{}", self.headline, self.location)
    }
}

// 3 trait作为参数
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 3.1  Trait Bound 语法
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

