struct Application {
    name: String,
    nicknames: Vec<String>
}

impl Drop for Application {
    fn drop(&mut self) {
        print!("Dropping {}", self.name);
        if !self.nicknames.is_empty() {
            print!(" (AKA {})", self.nicknames.join(", "));
        }

        println!("");
    }
}

fn main() {
    println!("Hello, world!");
}
