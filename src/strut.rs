use std::ops::{Bound, RangeBounds};

pub trait StringUtils 
{
    fn substring(&self, start: usize, len: usize) -> &str;
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;
}

impl StringUtils for str 
{
    fn substring(&self, start: usize, len: usize) -> &str 
    {
        let mut char_pos = 0;
        let mut byte_start = 0;
        let mut it = self.chars();
        loop 
        {
            if char_pos == start { break; }
            if let Some(c) = it.next() 
            {
                char_pos += 1;
                byte_start += c.len_utf8();
            }
            else { break; }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop 
        {
            if char_pos == len { break; }
            if let Some(c) = it.next() 
            {
                char_pos += 1;
                byte_end += c.len_utf8();
            }
            else { break; }
        }
        &self[byte_start..byte_end]
    }
    fn slice(&self, range: impl RangeBounds<usize>) -> &str 
    {
        let start = match range.start_bound() 
        {
            Bound::Included(bound) | Bound::Excluded(bound) => *bound,
            Bound::Unbounded => 0,
        };
        let len = match range.end_bound() 
        {
            Bound::Included(bound) => *bound + 1,
            Bound::Excluded(bound) => *bound,
            Bound::Unbounded => self.len(),
        } - start;
        self.substring(start, len)
    }
}

pub fn spaces(coo: f64) -> String
{
    let mut spaces: &str = "";

    if coo >= 0f64
    {
        if coo >= 100f64 {spaces = " ";}
        else if coo < 100f64 && coo >= 10f64 {spaces = "  ";}
        else if coo < 10f64 {spaces = "   ";}
    }
    else
    {
        if coo.abs() >= 100f64 {spaces = "";}
        else if coo.abs() < 100f64 && coo.abs() >= 10f64 {spaces = " ";}
        else if coo.abs() < 10f64 {spaces = "  ";}
    }
    
    spaces.to_string()
}