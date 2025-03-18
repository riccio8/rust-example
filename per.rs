struct Person{
    pub name: String,
    pub city: String,
    pub age: i32
}

impl Person {
    pub fn info(&self) {
        println!("Name: {}, City: {}, Age: {}", self.name, self.city, self.age);
    }
}

fn main(){
   let riccio = Person{
       name: "riccio".to_string(),
       city: "rove".to_string(),
       age: 15
   };
   riccio.info();
}

