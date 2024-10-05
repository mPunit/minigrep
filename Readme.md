### Command-Line-Program

This is an I/O Project: Building a Command Line Program

Basically, we will be rebuilding our own `grep` command for the terminal. Functionality-wise this won't add too much however this will give us a good understanding of how we can interact with the command line utility by creating our own commands.

For simplicity, we will be working with string type argument only i.e. UTF-8. So using the std::env::args to retrieve the command line arguments passed to the program as String values. 

The program can accept three arguments:
1) Query which will be our search string
2) File path
3) Flags (Optional)

Basic Command
```
cargo run -- <search_string> <file_path> <FLAGS>
```

### Functionality
`Case Sensitive/Insensitive search`
The program does the case sensitive as well as insensitive search. By default, it will be doing case sensitive searches. This programs allow two options: using the `Flags` or `environment variables` for the case sensitive/insensitive search.

- If you want to do insensitive search then you will need to set the Env var `IGNORE_CASE` in your terminal session with some value.
```
IGNORE_CASE=1 cargo run -- <search_string> <file_path>
```

- You can also use the `FLAGS` to achieve same

`--case_insensitive` - performs case insensitive search
`--case_sensitive` - perform case sensitive search

NOTE that  The flags will take precedence if both are present simultaneously. For example, with the below command, FLAG is taking precedence over the Env var to perform appropriate search i.e. case sensitive. 
```
IGNORE_CASE=1 cargo run are ../sample_file.txt --case_sensitive
```
