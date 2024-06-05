/**leetcod 3
 * 
*/
use std::collections::HashMap;
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut left = 0;
    let mut length = 0;
    let mut hash = HashMap::<char,i32>::new();
    for (right,value) in s.chars().into_iter().enumerate()
    {
        *hash.entry(value).or_insert(0) +=1;
        while *hash.get(&value).unwrap() > 1{
            let char_in_s = s.chars().nth(left).unwrap();
            * hash.entry(char_in_s).or_default()-=1;
            left +=1;
        }
        length = length.max((right-left+1)as i32);
    }
    length
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    pub fn leedcode_3_test()
    {
        let s = String::from("abcabcbb");
        println!("{}",length_of_longest_substring(s));
    }
}