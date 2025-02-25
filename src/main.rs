use std::ffi::c_int;

fn main() {

}

pub fn is_palindrome(word: String) -> bool{
    //create version of word spelled forwards and one spelled backwards, filter numbers, convert all characters to lowercase
    let mut letters = word.chars().filter(char::is_alphanumeric).map(char::to_ascii_lowercase);            // -----------------+-'a
    let mut letters_reverse = letters.rev().filter(char::is_alphanumeric).map(char::to_ascii_lowercase);   //-------------+-'b |
                                                                                                                                 //             |    |
    while let (Some(left), Some(right)) = (letters.next(), letters_reverse.next()){                                        //---+-d'--+-'c|    |
        //both characters (left and right) must be equal, or word is not a palindrome                                            //   |     |   |    |
        if left != right{                                                                                                        //   |     |   |    |
            return false;                                                                                                        //   |     |   |    |
        }                                                                                                                        //   |     |   |    |
    }                                                                                                                            //---+-----+   |    |
    true                                                                                                                         //             |    |
}                                                                                                                                //-------------+----+

pub fn delete_a_letter(word: String, index: usize) -> String{
    let mut letters: Vec<char> = word.chars().collect(); //convert to mutable data type

    //do not perform deletion at OOB index
    if index < letters.len() {
        letters.remove(index);
    }

    letters.into_iter().collect() //convert back to string, return changed string
}

