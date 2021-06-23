use std::collections::HashMap;

pub struct Data{
    pub data: HashMap<String,i32>,
}

impl Data{
    fn new()-> Data{
        let data_temp = HashMap::new();
        Data{
            data : data_temp,
        }
    }
}

fn main(){
    println!("Hi!");
}

