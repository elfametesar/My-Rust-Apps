use std::{env, fs, io, io::prelude::*};

fn help() {
    println!("{}", r#"
 Usage: sort { options } { sort type } { data to be sorted }
 
 Options:

  -r, --reverse Reverses the order of sort process
  
  -f, --file    Reads from a file and then sorts
                the data line by line.
 
 Parameters:
 
  -c, --char    Compares every single character 
                in a given string and sorts accordingly
 
  -w, --word    Takes in each word and sorts
                that word's letters amongst each other
 
  -n, --number  Number sorting, both positive and
                negative numbers
 
  -t, --text    Compares words amongst each other                    
                and sorts accordingly
 
 Arguments are expected after data type specifiers such as char,
 word, number etc.. Options such as reverse must come before the 
 data specifiers.

 As default, this app sorts each line in itself. If you need to
 sort the whole file content, you need to pass file contents as
 one whole string. Example:

    In Bash: sorted -n $(<numbers.txt)

 You can also achieve the same result by removing newline chars
 in other languages.
"#
);
}

fn sort_num(data: Vec<String>, reverse: bool) {
    let mut no_digit = false;
    for line in data.iter() {
        let mut sorted_line: Vec<f64> = Vec::new();
        for word in line.split_whitespace() {
            if word.parse::<f64>().is_ok(){
                sorted_line.push(word.parse().unwrap());
                no_digit = false;
            }
            else { no_digit = true }
        }
        sorted_line.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if reverse { sorted_line.reverse() }
        let sorted_string: String = sorted_line.iter().map(|s| format!("{} ", s.to_string())).collect();
        if !no_digit { println!("{}", sorted_string); }
    }
}

fn sort_char(data: Vec<String>, reverse: bool) {
    for line in data.iter() {
        let mut sorted_line: Vec<&str>  = line.split("").collect();
        sorted_line.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        if reverse { sorted_line.reverse() }
        println!("{}", sorted_line.join(""));
    }
}

fn sort_word(data: Vec<String>, reverse: bool) {
    for line in data.iter() {
        for word in line.split(" ") {
            let mut sorted_word: Vec<&str>  = word.split("").collect();
            sorted_word.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
           if reverse { sorted_word.reverse() }
            print!("{} ", sorted_word.join(""));
        }
        println!("");
    }
}

fn sort_text(data: Vec<String>, reverse: bool) {
    for line in data.iter() {
        let mut sorted_line: Vec<&str> = line.split(" ").collect();
        sorted_line.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        if reverse { sorted_line.reverse() }
        println!("{}", sorted_line.join(" "));
    }
}

fn read_from_stdin() -> Vec<String> {
    let mut data: Vec<String> = Vec::new();
    for line in io::stdin().lock().lines() {
        let get_line = line.unwrap();
        data.push(get_line.split("\n").map(|x| x.to_string()).collect());
    }
    return data
}

fn get_data(args: &Vec<String>, mut data: Vec<String>, index: usize) -> Vec<String> {
    match args.len() {
        x if x < index+2 => {
            if data.len() == 0 {
                data = read_from_stdin();
            }
        },
        _ => {
            data = vec![args[index+1..].join(" ").to_owned()];
      },
    }
    return data
}

fn main() {
    let mut reverse = false;
    let args: Vec<_> = env::args().collect();
    let mut data: Vec<String> = Vec::new();
    for (index, arg) in args.iter().enumerate() {
        if index == 0 { continue; }
        match arg.as_str() {
            "-h"|"--help" => { help(); return; },
            "-r"|"--reverse" => { reverse = true; continue; },
            "-f"|"--file" => {
                let file = fs::read_to_string(&args[index+1])
                    .expect("File can not be read!");
                data = file.split("\n")
                    .map(|s| s.to_string()).collect();
                continue;
            },
            "-n"|"--number" => {
                data = get_data(&args, data, index);
                sort_num(data, reverse); 
                return
            },
            "-t"|"--text" => {
                data = get_data(&args, data, index);
                sort_text(data, reverse);
                return
            },
            "-c"|"--char" => {
                data = get_data(&args, data, index);
                sort_char(data, reverse);
                return
            },
            "-w"|"--word" => {
                data = get_data(&args, data, index);
                sort_word(data, reverse);
//                 print!("{}", data.join(" "));
                return
            },
            _ => {
                if arg.starts_with("-") {
                    println!("{} parameter does not exist!", arg.as_str());
                }
            },
        }
    }
//     let rev = true;
//     let args = sort_numbers(&args, rev);
//     let text = fs::read_to_string(test).unwrap();
}
