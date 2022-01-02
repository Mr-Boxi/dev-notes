// 高效处理重复概念的工具
// 泛型

// 简单的代码服用： 使用函数
// 1. 在函数中定义使用泛型
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    // 为开启 std::cmp::PartialOrd 功能
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 2. 在结构体中定义泛型
struct Point<T> {
    x: T,
    y: T,
}


// 3. 枚举中定义泛型
enum Option<T> {
    Some(T),
    None,
}

// 4. 方法中定义泛型
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

