struct Account {
    name: String,
    location: String,
    age: u32,
}

#[test]
fn test_ref_pattern() {
    let account = Account {
        name: "keisuke".to_string(),
        location: "jp".to_string(),
        age: 10,
    };

    let result = match account {
        Account {
            // 値を移動するのではなく、借用している
            ref name,
            ref location,
            ..
        } => {
            println!("name: {}, location: {}", name, location);
            println!("{}", account.name)
        }
    };
}
