# Post Interview Project

In this project, I decided to have a struct called FileInfo that implements a trait called ContentInfo.

## ContentInfo Trait

The logic behind this trait was that it does not matter what the content, whether it be a file or just a regular String type, we would be able to call methods to count the numbeer of characters or to find the occurrence of a specific word. <br>

I implemented this trait to contain three methods: <br>
- `content_string`
  - `content_string` does not have a default implementation so any types that want to implement ContentInfo would be required to specify their definition of this method. `content_string` returns a string that can be used in other methods in the trait.<br>
- `count_char`
  - `count_char` does have a default implementation. The method makes use of the `content_string`. `content_string` returns a String, which can be used to count the amount of characters in that string. <br>
- `find_word`
  - `find_word` also has a default implementation that makes uses of `content_string`. It uses the string returned from `count_char` to return the number of occurrences of a specified word in the string. <br>

## FileInfo Struct

FileInfo has a member variable called `filename` which is a type `String`. It can be expanded to include other file metadata. The FileInfo struct implements ContentInfo. Thus, it is required to implement the content_string method. In the FileInfo `content_string` mothod, it takes the filename member of the struct and attempts to open the file and return the contents of the file as a String. If the method fails to open or read the file, the method would cause the program to panic. The `content_string` method allows the FileInfo struct to have the capability to call other methods in the trait since they have default implementations, unless those methods are overwritten. 

## Main function

The main function allows us to use the command line to take a command and execute the logic of reading, counting, and find, while utilizing the FileInfo struct to handle the commands to print the information to the console.

## Unit testing

For the `content_string` method, I included two tests: one that tests if it can return a string if the file exists and another tests to see if the method panics if the file does not exist. For the other methods, I included two asserts to see if the method works on different files (`hello.txt` and `multi_line_file.txt`).
