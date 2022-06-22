

struct bird {
    name: String,
    age: u32,
    size: String
}

impl bird {
    fn print_name(&self) {
        println!("{}", self.name);
    }
}

fn first <T> (slice: &[T]) -> &T {
    &slice[0]
}

impl dinosaurs for bird {
    fn can_fly(&self) -> bool {
        true
    }
}
fn main() {
    let birdy = bird { name: "birdy".to_string(), age: 25, size: "small".to_string() };
    birdy.print_name();
    print!("{}", birdy.age);

}
trait dinosaurs {
    fn can_kill_you(&self) -> bool {
        true
    }
    fn can_fly(&self) -> bool;

}