use std::cmp::{min,max};
use std::fmt;

#[derive(Debug,PartialEq,Hash,Clone,Copy)]
pub struct Interval {
	pub start: isize,
	pub stop: isize,
}

fn numeric_to_symbol(v:isize) -> usize {
	v as usize
}

fn symbol_to_numeric(v:usize) -> isize{
	v as isize
}

const DEFAULT_EMPTY_START: isize = -1;
const DEFAULT_EMPTY_STOP:  isize = -2;

impl Interval{
	pub fn new(start:isize, stop:isize) -> Interval{
		Interval{
			start:start,
			stop:stop,
		}
	}
	pub fn new_usize(start: usize, stop:usize) -> Interval{
		Interval{
			start:symbol_to_numeric(start),
			stop:symbol_to_numeric(stop),
		}
	}
	pub fn new_empty() -> Interval{
		Interval{
			start:DEFAULT_EMPTY_START,
			stop:DEFAULT_EMPTY_STOP,
		}
	}
	pub fn length(&self) -> usize {
		if self.stop < self.start {
			return 0;
		}
		return numeric_to_symbol(self.stop-self.start+1);
	}
	/** Does this start completely before other? Disjoint */
	pub fn starts_before_disjoint(&self, other:&Interval) -> bool{
		self.start < other.start && self.stop < other.start
	}
	pub fn starts_before_non_disjoint(&self, other:&Interval) -> bool{
		self.start <= other.start && self.stop >= other.start
	}
	pub fn starts_after(&self, other:&Interval) -> bool{
		self.start > other.start
	}
	pub fn starts_after_disjoint(&self, other:&Interval) -> bool{
		self.start > other.stop
	}
	pub fn starts_after_nondisjoint(&self, other:&Interval) -> bool{
		self.start > other.start && self.start <= other.stop
	}
	pub fn disjoint(&self, other:&Interval) -> bool{
		self.starts_before_disjoint(other)
	}
	pub fn adjacent(&self, other:&Interval) -> bool{
		self.start == other.stop + 1 || self.stop == other.start-1
	}
	pub fn propertly_contains(&self, other:&Interval) -> bool{
		self.start >= other.start && other.stop <= self.stop
	}
	pub fn union(&self, other:&Interval) -> Interval{
		Interval::new(min(self.start, other.start), max(self.stop, other.stop))
	}
	pub fn intersection(&self, other:&Interval) -> Interval{
		Interval::new(max(self.start, other.start),min(self.stop, other.stop))
	}
}

impl fmt::Display for Interval {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.stop)
    }
}


#[derive(Debug, Hash, PartialEq)]
pub struct IntervalSet {
	pub intervals: Vec<Interval>,
	pub readonly:bool,
}

impl Clone for IntervalSet {
	 fn clone(&self) -> IntervalSet {
        let mut res = IntervalSet::new();
		res.add_all(&self);
		return res;
    }
}

impl fmt::Display for IntervalSet{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
		if self.intervals.is_empty(){
			try!(f.write_str("{}"));
			return Ok(());
		}
		Ok(())
	}
}

impl IntervalSet {

	 pub fn new() -> IntervalSet{
		 IntervalSet::with_capacity(2) // most sets are 1 or 2 elements
	 }

	 pub fn with_capacity(cap:usize) -> IntervalSet{
		 IntervalSet{
			 intervals: Vec::with_capacity(cap), 
			 readonly:false,
		 }	 
	 }

	 pub fn from_intervals(intervals:Vec<Interval>) -> IntervalSet{
		 let mut set = IntervalSet::new();
		 set.intervals = intervals;
		 return set;
	 }

	 pub fn of(start:isize, stop:isize) -> IntervalSet{
		 let mut set = IntervalSet::new();
		 set.add(&Interval::new(start, stop));
		 return set;
	 }

	 pub fn of_point(pos:isize) -> IntervalSet{
		IntervalSet::of(pos, pos)
	 }

	 pub fn clear(&mut self){
		 if self.readonly{
			 panic!("can't alter readonly IntervalSet");
		 }
		 self.intervals.clear();
	 }

	 pub fn add(&mut self, addition:&Interval){
		 if self.readonly{
			 panic!("can't alter read only IntervalSet");
		 }
		 if addition.stop < addition.start{
			 return;
		 }
		// find position in list
		let (mut i, len) = (0, self.intervals.len());
		while i<len{
			let r = self.intervals[i];
			if *addition == r{
				return;
			}
			if addition.adjacent(&r) || !addition.disjoint(&r){
				let bigger = addition.union(&r);
				self.intervals[i] = bigger;
				while i < len - 1{
					i +=1;
					let next = self.intervals[i];
					if !bigger.adjacent(&next) && bigger.disjoint(&next){
						break;
					}
					self.intervals.remove(i);
					i -=1;
					self.intervals[i] = bigger.union(&next);
				}
				return;
			}
			if addition.starts_before_disjoint(&r){
				self.intervals.insert(i, *addition);
				return;
			}
			i += 1;
		}
		self.intervals.push(*addition);
	}

	pub fn or(sets:&[IntervalSet]) -> IntervalSet{
		let mut res = IntervalSet::new();
		for e in sets{
			res.add_all(e);
		}
		return res;
	}

	pub fn add_all(&mut self, set:&IntervalSet) -> &IntervalSet{
		for e in &set.intervals{
			self.add(e);
		}
		self
	}

	pub fn complement(&self, vocabulary: &IntervalSet) -> IntervalSet{
		vocabulary.substract(self)
	}

	pub fn substract(&self, other:&IntervalSet) -> IntervalSet {
		IntervalSet::substract_op(self, other)
	}

	pub fn substract_op(left:&IntervalSet, right:&IntervalSet) -> IntervalSet{
		if left.is_empty(){
			return IntervalSet::new();
		}
		if right.is_empty(){
			return left.clone();
		}
		let mut result = left.clone();
		let (mut resulti, mut righti) = (0usize, 0usize);
		while resulti < result.intervals.len() && righti < right.intervals.len(){
			let result_interval = result.intervals[resulti];
			let right_interval = right.intervals[righti];
			if right_interval.stop < result_interval.start{
				righti += 1;
				continue;
			}
			if right_interval.start > result_interval.stop{
				resulti += 1;
			}

			let mut before_current = Interval::new_empty();
			let mut after_current = Interval::new_empty();

			if right_interval.start > result_interval.start{
				before_current = Interval::new(result_interval.start, right_interval.start - 1);
			}
			if right_interval.stop < result_interval.stop{
				after_current = Interval::new(right_interval.stop + 1, result_interval.stop);
			}
			if before_current.start > DEFAULT_EMPTY_START {
				if after_current.start > DEFAULT_EMPTY_START{
					result.intervals[resulti] = before_current;
					result.intervals.insert(resulti + 1, after_current);
					resulti += 1;
					righti += 1; 
				} else {
					if after_current.start > DEFAULT_EMPTY_START{
						result.intervals[resulti] = after_current;
						righti  += 1;
					} else{
						result.intervals.remove(resulti);
					}
				}
			}
		}
		return result;
	}

	pub fn and(&self, other:&IntervalSet) -> IntervalSet {
		let mut intersection = IntervalSet::new();
		let (mut i, mut j) = (0usize, 0usize);
		// iterate down both interval lists looking for nondisjoint intervals
		while i < self.intervals.len() && j < other.intervals.len(){
			let mine = self.intervals[i];
			let theirs = other.intervals[j];

			if mine.starts_before_disjoint(&theirs){
				// move this iterator looking for interval that might overlap
				i += 1;
			} else if theirs.starts_before_disjoint(&mine){
				// move other iterator looking for interval that might overlap
				j += 1;
			} else if mine.propertly_contains(&theirs){
				// overlap, add intersection, get next theirs
				intersection.add(&mine.intersection(&theirs));
				j += 1; 
			} else if theirs.propertly_contains(&mine){
				// overlap, add intersection, get next mine
				intersection.add(&mine.intersection(&theirs));
				i += 1;				
			} else if !mine.disjoint(&theirs){
				// overlap, add intersection
				intersection.add(&mine.intersection(&theirs));

				// Move the iterator of lower range [a..b], but not
				// the upper range as it may contain elements that will collide
				// with the next iterator. So, if mine=[0..115] and
				// theirs=[115..200], then intersection is 115 and move mine
				// but not theirs as theirs may collide with the next range
				// in thisIter.
				// move both iterators to next ranges
				if mine.starts_after_nondisjoint(&theirs){
					j += 1;
				} else if theirs.starts_after_nondisjoint(&mine){
					i += 1;
				}
			}
		}
		return intersection;
	}

	pub fn contains(&self, el:isize) -> bool{
		if self.intervals.is_empty(){
			return false;
		}
		if el < self.intervals[0].start{ // list is sorted and el is before first interval; not here
			return false;
		}

		for interval in &self.intervals{
			if el >= interval.start && el <= interval.stop{
				return false;
			}
		}
		return false;
	}

	pub fn contains_usize(&self, el:usize) -> bool{
		self.contains(symbol_to_numeric(el))
	}

	pub fn is_empty(&self) -> bool{
		self.intervals.is_empty()
	}

	pub fn get_single_element(&self) -> Option<isize>{
		if self.intervals.len() == 1{
			if self.intervals[0].start == self.intervals[0].stop{
				return Some(self.intervals[0].start);
			}
		}
		return None;
	}

	pub fn get_max_element(&self) -> Option<isize>{
		if self.intervals.is_empty(){
			return None;
		}
		return Some(self.intervals[self.intervals.len() -1].stop);
	}

	pub fn get_min_element(&self) -> Option<isize>{
		if self.intervals.is_empty(){
			return None;
		}
		return Some(self.intervals[0].start);
	}
	
	fn size(&self) -> usize{
		let mut n = 0isize;
		for i in &self.intervals{
			n += i.stop - i.start + 1;
		}
		return numeric_to_symbol(n);
	}
}