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
    let mut _a = Application { name: "Zeus".to_string(),
                               nicknames: vec!["cloud collector".to_string(),
                                               "king of the gods".to_string()] };
    println!("before assignment");
    _a = Application { name: "Hera".to_string(), nicknames: vec![] };
    println!("at end of block");
}
