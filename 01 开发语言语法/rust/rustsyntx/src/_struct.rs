//  结构体学习
//  结构体可以组织不同的数据


// 1. 定义结构体
#[derive(Debug)]
pub struct User {
    pub username: String,
    pub email: String,

} 
// 2. 实例化结构体
pub fn creat(){
    let user1 = User{
        username: String::from("boxi"),
        email: String::from("123@ss.com"),
    };
    println!("{:?}", user1);
}


// 3. 定义方法
// 第一个参数是self : self, &self, &mut self
#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}
impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

// 4. 关联函数
// 第一个参数不是self
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}