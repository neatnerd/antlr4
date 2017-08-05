#![allow(dead_code)]
pub trait Token:ToString {
    fn get_text(&self) -> String; 
    fn get_type(&self) -> usize;
    fn get_line(&self) -> usize;
    fn get_char_position_in_line(&self) -> usize;
    fn get_channel(&self) -> usize;
    fn get_token_index(&self) -> usize;
    fn get_start_index(&self) -> usize;
    fn get_stop_index(&self) -> usize;
    fn get_token_source(&self) -> &TokenFactory;
    fn get_input_stream(&self) -> &CharStream;  
}

pub trait TokenSource {
    fn next_token(&mut self) -> Token;
    fn get_line(&self) -> usize;
    fn get_char_position_in_line(&self) -> usize;
    fn get_input_stream(&self) -> &CharStream;
    fn get_source_name(&self) -> String;
    fn set_token_factory(&mut self, &TokenFactory);
    fn get_token_factory(&self) -> &TokenFactory;
}

pub struct Pair<'a, A:'a+?Sized, B:'a+?Sized>(&'a A, &'a B);

pub trait TokenFactory {
    fn create(&self, source : Pair<TokenSource, CharStream>, _type: usize, text:String,
                channel: usize, start:usize, stop:usize, line:usize, char_position_in_line:usize);
}


pub trait CharStream:ToString {
        fn get_text(&self, interval:&Interval) -> String;
}

pub trait Interval {
        
}

pub trait AntlrErrorListener {
     fn syntax_error(recognizer:&Recognizer, offending_symbol:&Token,
     line:usize, char_position_in_line:usize, msg:&'static str/*,std::exception_ptr e */);
}

pub trait Recognizer {
     
}

pub trait Parser {

}

