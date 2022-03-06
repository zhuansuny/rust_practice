fn main() {
    // 迭代器  结构体实现了trait Iterator 就可以使用迭代器
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    // for val in v1_iter {
    //     println!("{}",val)
    // }

    // 消耗迭代器方法 ， 方法中调用了 next方法
    let sum: i32 = v1_iter.sum();
    println!("sum is {}", sum);
}
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // 闭包获取上下文环境
    // into_iter 获取所有权
    // filter 传入闭包函数进行过滤，返回 false过滤掉
    shoes.into_iter().filter(|x| x.size == shoe_size).collect()
}

// 创建自定义的迭代器

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {count: 0}
    }
    
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        }else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{shoes_in_my_size, Shoe, Counter};

    #[test]
    fn iterator_domonstration() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();
        // 实现 next 方法就可以用迭代器
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
    }

    #[test]
    fn iter_sum() {
        let v1: Vec<i32> = vec![1, 2, 3];
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v1, vec![1, 2, 3]);
        assert_eq!(v2, vec![2, 3, 4]);
    }

    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 20,
                style: String::from("sandal"),
            },
            Shoe {
                size: 20,
                style: String::from("boot"),
            },
        ];
        let in_my_size = shoes_in_my_size(shoes, 20);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 20,
                    style: String::from("sandal"),
                },
                Shoe {
                    size: 20,
                    style: String::from("boot"),
                }
            ]
        );
    }

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();


        assert_eq!(counter.next(),Some(1));
        assert_eq!(counter.next(),Some(2));
        assert_eq!(counter.next(),Some(3));
        assert_eq!(counter.next(),Some(4));
        assert_eq!(counter.next(),Some(5));
        assert_eq!(counter.next(),None);

        let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1)) //略过第一个元素
        .map(|(a,b)| a*b)
        .filter(|x| x%3 == 0)
        .sum();

        assert_eq!(sum,18);

    }
}
