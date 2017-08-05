#![allow(dead_code)]

use misc::interval::IntervalSet;
#[derive(Debug)]
pub enum SerializationType {
      EPSILON = 1,
      RANGE = 2,
      RULE = 3,
      PREDICATE = 4, // e.g., {isType(input.LT(1))}?
      ATOM = 5,
      ACTION = 6,
      SET = 7, // ~(A|B) or ~atom, wildcard, which convert to next 2
      NOTSET = 8,
      WILDCARD = 9,
      PRECEDENCE = 10,
}

static SERIALIZATION_NAMES: &'static [&str] = &["INVALID", "EPSILON", "RANGE", "RULE", "PREDICATE", "ATOM", "ACTION", "SET", "NOT_SET", "WILDCARD", "PRECEDENCE"];

pub trait Transition:ToString+Eq {
     fn new(target:&ATNState) -> Self;
     fn get_serialization_type(&self) ->SerializationType;
     fn is_epsilon(&self) -> bool{
        return false;
     }
     fn label(&self) -> IntervalSet;
         //TODO: return default empty interval set
         //return misc::IntervalSet::EMPTY_SET;

     fn matches(&self, symbol:usize, min_vocab_symbol:usize, max_vocab_symbol:usize) -> bool;
     fn to_string(&self) -> String{
        //TODO: ss << "(Transition " << std::hex << this << ", target: " << std::hex << target << ')';
        format!("(Transition )")
     }
}


pub struct ActionTransition {
    rule_index:         usize,
    action_index:       usize,
    is_ctx_dependent:   bool,
}
/*
impl ActionTransition {
     
}

impl Transition for ActionTransition {

}
*/
pub trait ATNState {
    
}
