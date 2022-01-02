mod _struct;
mod _enum;
mod _function;
mod _generics;
mod _trait;


// fn main() {

//     _struct::creat();
//     println!("Hello, world!");

//     let rect1 = _struct::Rectangle{
//         width: 30,
//         height: 30,
//     };
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );

// }
fn main() {
    let number_list = vec![34,25,100];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("the largest number is {}", largest);
    

}
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}