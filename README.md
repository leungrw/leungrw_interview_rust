# Post Interview Project

In this project, I decided to have a struct called FileInfo that implements a trait called ContentInfo.

## ContentInfo Trait

The logic behind this trait was that it does not matter what the content, whether it may be a file or just a regular String type, is we would be able to call methods like counting the characters in the content or counting the number of the specified string. <br>

I implemented this trait to contain three methods: <br>
- content_string
  - This method does not have a default implementation so any types that want to implement the ContentInfo trait would be required to specify their definition of this method. content_string also returns a string that can be used in other methods in the trait.<br>
- count_char
  - count_char does have a default implementation. In the default implementation of the method, the method makes use of the content_string. Since we know that content_string returns a String, we can make use of that to then count the amount of characters in that string. <br>
- find_word
  - Like count_char, find_word also has a default implementation that makes uses of content_string. In the same logic as count_char, we can make use of the String returned in count_char to then find the matching word and return the number of occurrences a specified word occur in the string. <br>



## FileInfo Struct

Currently FileInfo only has a member variable called filename, which is a String, but can be expanded later on to include other metadata about files. As mentioned earlier, the FileInfo struct implements ContentInfo. Thus, it is required to implement the content_string method. In the content_string method under FileInfo, the implementation takes the filename member of the struct to attempt to open the file and return the contents of the file into a String. However, if the method fails to open or read the file for any reason, the method would cause the program to panic. By implementing the content_string method, the FileInfo struct would also have the capability to call the other methods in the trait since they have default implementations, unless those methods are overwritten. 

## Main function

The main function still allows us to use the command line to take a command and execute the logic of reading, counting, and find, but utilizes the FileInfo struct to handle the commands to print the information to the console.

## Unit testing

For the three methods that are defined in FileInfo and Content, I have also created unit tests for them. For the content_string method, I includeds two test: one that tests checks if it can return a string if the file exists and another to test to see if the method panics if the file does not exist. For the other methods, I included two asserts to see if the method works on different files (hello.txt and multi_line_file.txt).
