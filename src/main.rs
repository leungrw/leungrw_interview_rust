use std::env;
use std::fs;

pub trait ContentInfo {
    fn content_string(&self) -> String;

    fn count_char(&self) -> i32 {
        let mut count = 0;

        for _c in self.content_string().chars() {
            count+=1;
        }

        count 
    }

    fn find_word(&self, word: String) -> i32 {
        let content = self.content_string();
        let filtered: Vec<&str> = content.split_whitespace().filter(|x| *x==word).collect();
        filtered.len() as i32
    }
}


pub struct FileInfo {
    filename: String,
}

impl ContentInfo for FileInfo{
    fn content_string(&self) -> String{
        match fs::read_to_string(&self.filename){
            Ok(item) => item,
            Err(_) => panic!("Unable to open and read file: {}", self.filename),
        }
    }

}


fn main() {
    let args : Vec<String> = env::args().collect();

    let command = &args[1];

    if command == "read" {
        // let content = read_file(args[2].clone());
        let file = FileInfo {
            filename: args[2].clone(),
        };
        println!("{}", file.content_string());
    } else if command == "count" {
        // let count = count_char(args[2].clone());
        let file = FileInfo {
            filename: args[2].clone(),
        };
        println!("{}", file.count_char());
    } else if command == "find" {
        // let found = find(args[2].clone(), args[3].clone());
        let file = FileInfo {
            filename: args[2].clone(),
        };
        println!("{}", file.find_word(args[3].clone()));
    } else {
        panic!("Invalid command: recived {:?}", args);
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[should_panic]
    fn content_string_non_existant_file() {
        let non_existant_file = FileInfo {
            filename: String::from("does_not_exist.txt"),
            // file_content: String::from(""),
        };
        non_existant_file.content_string();
    }

    #[test]
    fn content_string_existant_file() {
        let hello_file = FileInfo{
            filename:String::from("hello.txt"),
            // file_content: String::from(""),
        };
        let multi_line_file = FileInfo{
            filename:String::from("multi_line_file.txt"),
        };
        assert_eq!(hello_file.content_string(), String::from("hello how are you"));
        assert_eq!(multi_line_file.content_string(), String::from("This is a test file.\nThere is more than one line."));
    }

    #[test]
    fn counting_char_of_file() {
        let hello_file = FileInfo{
            filename:String::from("hello.txt"),
            // file_content: String::from(""),
        };
        let multi_line_file = FileInfo{
            filename:String::from("multi_line_file.txt"),
        };
        assert_eq!(hello_file.count_char(), 17);
        assert_eq!(multi_line_file.count_char(), 49);
    }

    #[test]
    fn matching_word_to_file() {
        let hello_file = FileInfo{
            filename:String::from("hello.txt"),
        };
        let multi_line_file = FileInfo{
            filename:String::from("multi_line_file.txt"),
        };
        assert_eq!(hello_file.find_word(String::from("you")), 1);
        assert_eq!(multi_line_file.find_word(String::from("is")), 2);
    }
}
