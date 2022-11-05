trait Hello {
    fn say(&self, name: &str);
}

struct Japanese {
    word: String,
}

struct English {
    word: String,
}

impl Hello for Japanese {
    fn say(&self, name: &str) {
        println!("{}、{}", self.word, name);
    }
}

impl Hello for English {
    fn say(&self, name: &str) {
        println!("{},{}", self.word, name);
    }
}

fn say_hello(f: impl Hello, name: &str) {
    f.say(name);
}

fn main() {
    let j = Japanese {
        word: "こんにちは".to_string(),
    };
    let e = English {
        word: "Hello".to_string(),
    };

    say_hello(j, "takeshi");
    say_hello(e, "takeshi");
}
