trait Hello {
    fn say(&self, name: String) -> String;
}

struct English {
    word: String,
}

impl Hello for English {
    fn say(&self, name: String) -> String {
        (&self.word).to_string() + ", " + name.as_str()
    }
}

fn main() {
    let e = English {
        word: "hello".to_string(),
    };

    println!("{}", e.say("takeshi".to_string()));
}
