fn main() {
    let number_list = vec![12,34,56,78,90];
    println!("the largest number is {}",largest(&number_list));

    let p = Point{x:5,y:6};
    println!("p.x = {}", p.x());

    let p = Point{x:5.0,y:6.0};
    println!("p.distance_from_origin = {}",p.distance_from_origin());


}

struct Point<T> {
    x:T,  //可以是任意类型 但x和y的类型要相同
    y:T,
}

struct PointV2<T,F> {
    x:T,  //可以是任意类型 但x和y的类型可以不相同
    y:F,
}

// 所有类型都有的方法
impl<T> Point<T> {
    fn x(&self) -> &T{
        &self.x
    }
}

// 只有f32类型才有的方法
impl Point<f32> {
    fn distance_from_origin(&self) -> f32{
        (self.x.powi(2)+self.y.powi(2)).sqrt()
    }
}



// 不同类型的数组取最大值
fn largest_i32 (list : &[i32]) -> i32 {
    if list.len() == 0 {
        return 0
    }
    let mut largest  = list[0];

    for &item in list {
        if item > largest {
            largest = item
        }
    };
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

// 将上面的两个方法用范型合并
fn largest<T: std::cmp::PartialOrd>(list : &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}