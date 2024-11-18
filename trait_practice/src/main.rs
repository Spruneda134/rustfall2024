pub trait Talkative {
    fn talk (&self) {
        println!("Hello my name is {}", self.name);
    }
}


struct Person {
    name: str
}

impl Person {
    fn talk for Person(&self) {
        println!("Hello my name is {}", self.name);
    }
}


fn main() {
    let me = Person{name: "salvador".to_string()};
    me.talk();

    let mut v = vec![s];
}

hard code the prices
create 3 structs, data should just be usd price
gold
bitcoin
sp500

trait priceitem, getPrice()
each should extract their own priceitem
make api classic_example_stack


put them in the same vector
every 10 seconds call it and update prices
