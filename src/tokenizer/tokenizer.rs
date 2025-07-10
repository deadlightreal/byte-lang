pub struct Tokenizer<'a> {
    content: Vec<&'a str>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a String) -> Self {
        let content : Vec<&'a str>  = input.lines().collect();
        return Tokenizer { content };
    }
}
