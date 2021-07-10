pub trait Ided {
    fn my_id(&self) -> i64;
}

pub fn use_idea(x: Box<dyn Ided>) {
    println!("x id is {}", x.my_id());
}
