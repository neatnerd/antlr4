use std::cmp;
use definitions::Token_Type;

#[derive(Debug,PartialEq)]
pub struct Vocabulary {
    literal_names:  Vec<String>,
    symbolic_names: Vec<String>,
    display_names:  Vec<String>,
    max_toke_type:  usize,
}

/*
lazy_static! {
    static ref EMPTY_VOCABULARY: Vocabulary = Vocabulary::empty();
}

pub fn empty_vocabulary() -> &'static Vocabulary {
    &*EMPTY_VOCABULARY
}
*/  

impl Vocabulary{
    pub fn new(literal_names:&[String], symbolic_names:&[String],
               display_names:&[String]) -> Vocabulary {
        let mut res = Vocabulary::empty();
        res.literal_names.extend_from_slice(literal_names);
        res.symbolic_names.extend_from_slice(symbolic_names);
        res.display_names.extend_from_slice(display_names);
        res.max_toke_type = cmp::max(cmp::max(res.literal_names.len(), res.symbolic_names.len()), res.display_names.len()) - 1;
        return res;
    }
    pub fn empty() -> Vocabulary{
        Vocabulary{
            literal_names:  Vec::new(),
            symbolic_names: Vec::new(),
            display_names:  Vec::new(),
            max_toke_type:  0,            
        }
    }

    pub fn from_token_names(token_names: &Vec<String>) -> Vocabulary{
        if token_names.is_empty(){
            return Vocabulary::empty();
        }
        let (mut literal_names, mut symbolic_names) = (token_names.clone(), token_names.clone());
        for i in 0..token_names.len(){
            let token_name = &token_names[0];
            if token_name.is_empty(){
                continue;
            }
            let first_char = token_name.chars().nth(0).unwrap();
            if first_char == '\''{
                symbolic_names[i] = String::from("");
                continue;
            } else if first_char.is_uppercase(){
                literal_names[i] = String::from("");
                continue;
            }

            literal_names[i] = String::from("");
            symbolic_names[i] = String::from("");
        }
        return Vocabulary::new(&literal_names, &symbolic_names, &token_names);
    }

    pub fn get_max_toke_type(&self) -> usize{
        self.max_toke_type
    }

    pub fn get_literal_name(&self, token_type:&Token_Type) -> &str{
        if let Token_Type::USER(token) = *token_type{
            if token < self.literal_names.len(){
                return self.literal_names[token].as_str();
            }
        }
        return "";
    }

    pub fn get_symbolic_name(&self, token_type:&Token_Type) -> &str{
        match *token_type{
            Token_Type::EOF => "EOF",
            Token_Type::USER(token) if token < self.symbolic_names.len() => self.symbolic_names[token].as_str(),
            _ => ""
        }
    }

    pub fn get_display_name(&self, token_type:&Token_Type) -> &str{
        if let Token_Type::USER(token) = *token_type{
            if token < self.display_names.len(){
                return self.display_names[token].as_str();
            }
        }
        let literal_name = self.get_literal_name(token_type);
        if !literal_name.is_empty(){
            return literal_name;
        }
        let symbolic_name = self.get_symbolic_name(token_type);
        if !symbolic_name.is_empty(){
            return symbolic_name;
        }
        return "";
    }

}
