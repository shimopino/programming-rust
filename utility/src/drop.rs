struct Application {
    name: String,
    nicknames: Vec<String>,
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

#[test]
fn test_custom_drop() {
    let mut a = Application {
        name: "shimokawa".to_string(),
        nicknames: vec!["cloud".to_string(), "king".to_string()],
    };

    println!("Before assignment");
    a = Application {
        name: "keisuke".to_string(),
        nicknames: vec!["replaced".to_string()],
    };
    println!("at end of block")
}
