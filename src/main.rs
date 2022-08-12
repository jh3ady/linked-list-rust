use std::fmt::Display;

#[derive(Debug, Clone)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<
    T: Display + Clone
> List<T> {
    pub fn new() -> Self {
        List::Nil
    }

    pub fn prepend(self, item: T) -> Self {
        List::Cons(item, Box::new(self))
    }

    pub fn append(self, item: T) -> Self {
        match self {
            List::Nil => List::Cons(item, Box::new(List::Nil)),
            List::Cons(x, xs) => List::Cons(x, Box::new(xs.append(item))),
        }
    }

    pub fn stringify(&self) -> String {
        match self {
            List::Cons(x, xs) => format!("{}, {}", x, xs.stringify()),
            List::Nil => format!("Nil"),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            List::Cons(_, xs) => 1 + xs.len(),
            List::Nil => 0,
        }
    }

    pub fn append_list(self, other: List<T>) -> List<T> {
        match self {
            List::Cons(x, xs) => List::Cons(x, Box::new(xs.append_list(other))),
            List::Nil => other.clone(),
        }
    }

    pub fn prepend_list(self, other: List<T>) -> List<T> {
        other.append_list(self)
    }
}

fn main() {
    let list_1_2 = List::new()
        .append(1)
        .append(2);
    println!("list_1_2: {} ({})", list_1_2.stringify(), list_1_2.len());

    let list_3_4_5 = List::new()
        .append(3)
        .append(4)
        .append(5);
    println!("list_3_4_5: {} ({})", list_3_4_5.stringify(), list_3_4_5.len());

    let list_6_7 = List::new()
        .append(7)
        .prepend(6);
    println!("list_6_7: {} ({})", list_6_7.stringify(), list_6_7.len());

    let final_list = List::new()
        .append_list(list_3_4_5)
        .prepend_list(list_1_2)
        .append_list(list_6_7);
    println!("final_list: {} ({})", final_list.stringify(), final_list.len());
}
