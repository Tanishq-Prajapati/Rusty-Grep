// creating a CLI-Application
// Grep Command line

// no of operations i will perform
// 1. word search
// 2. starts with
// 3. ends with
// 4. word or search

use core::panic;
use std::env;
use std::fs;
use colorama::Colored;
enum Process{
    WORDSEARCH,
    STARTSWITH,
    ENDSWITH,
    TWOWORDSEARCH
}

struct Operation{
    file_data: String,
    identifier: String,
    process: Process
}

fn invalid_args_message(){
    println!("Invalid Arguments Given...\nCOMMAND EX: (program-execution) <FILENAME> <\"IDENTIFIER\">");
}

fn is_file_present(file_name: &String) -> bool{
    // trying to open the file
    match fs::File::open(file_name) {
        Ok(_) => true,
        _ => false
    }
}

fn normal_word_pattern_check(
    iden: &str,
    file_data: &str
) -> Vec<String>
{
    // now checking for the word here in each data line
    let file_data_list: Vec<&str> = file_data.split("\n").collect();
    let mut searched_data: Vec<String> = Vec::new();
    for a_data_line in file_data_list.iter(){
        // running a loop on the length of line_data
        for a_idx in 0..a_data_line.len(){
            if (a_idx + iden.len()) >= a_data_line.len(){
                if &a_data_line[a_idx..a_data_line.len()] == iden{
                    searched_data.push(a_data_line.to_string());
                }
            }
            else {
                if &a_data_line[a_idx..(a_idx + iden.len())] == iden{
                    searched_data.push(a_data_line.to_string());
                }
            }
        }
    }

    // prointing all the serched Data here
    for a_searched_line_idx in 0..searched_data.len(){
        searched_data[a_searched_line_idx] = searched_data[a_searched_line_idx].replace(iden, iden.to_string().color("red"));
    }

    return searched_data;
}

fn endswith_pattern_check(
    iden: &str,
    file_data: &str
) -> Vec<String>{
    let file_data_list: Vec<&str> = file_data.split("\n").collect();
    let real_iden = &iden[0..iden.len()-1];
    // running a loop on all the files_data here
    let mut searched_data: Vec<String> = Vec::new();
    for a_line in file_data_list{
        if a_line.ends_with(real_iden){
            searched_data.push(a_line.to_string());
        }
    }

    // prointing all the serched Data here
    for a_searched_line_idx in 0..searched_data.len(){
        searched_data[a_searched_line_idx] = searched_data[a_searched_line_idx].replace(real_iden, real_iden.to_string().color("red"));
    }

    return searched_data;
}

fn startswith_pattern_check(
    iden: &str,
    file_data: &str
) -> Vec<String>{
    let file_data_list: Vec<&str> = file_data.split("\n").collect();
    let real_iden = &iden[1..iden.len()];
    // running a loop on all the files_data here
    let mut searched_data: Vec<String> = Vec::new();
    for a_line in file_data_list{
        if a_line.starts_with(real_iden){
            searched_data.push(a_line.to_string());
        }
    }

    // prointing all the serched Data here
    for a_searched_line_idx in 0..searched_data.len(){
        searched_data[a_searched_line_idx] = searched_data[a_searched_line_idx].replace(real_iden, real_iden.to_string().color("red"));
    }

    return searched_data;
}

fn pipe_pattern_check(
    iden: &str,
    file_data: &str
) -> Vec<String>{
    // getting all the word searchers here
    let all_searchers: Vec<&str> = iden.split("|")
        .filter(|&x| !x.is_empty()).collect();

    // now iterating from the file-data here
    let file_data_list: Vec<&str> = file_data.split("\n").collect();
    let mut searched_data: Vec<String> = Vec::new();
    for a_line in file_data_list{
        // we will run a loop on all_searchers here
        let mut is_found = false;
        for a_searcher in all_searchers.iter(){
            for a_dat in 0..a_line.len() - 1{
                if !is_found && &&a_line[a_dat..(a_dat + a_searcher.len())] == a_searcher{
                    is_found = true;
                    searched_data.push(a_line.to_string());
                }
            }
        }
    }

    // prointing all the serched Data here
    for a_searched_line_idx in 0..searched_data.len(){
        for a_search in all_searchers.iter(){
            searched_data[a_searched_line_idx] = searched_data[a_searched_line_idx].replace(a_search, a_search.to_string().color("red"));
        }
    }

    return searched_data;
}

fn check_args(
    file_name: &String,
    identifier: &String
) -> (bool, String){
    println!("Checking...: {}",identifier);
    // Verifying the File here
    if !is_file_present(file_name) {
        return (false, "Error: File Not Found".to_string());
    }
    else if identifier.len() == 0 { 
        return (false, "Error: Identifier cannot be Blank".to_string()); 
    }
    
    // creating a array of chars here
    let iden_chars: Vec<char> = identifier.chars().collect();
    let mut pipe_present = false;
    for a_char in iden_chars.iter(){
        if a_char == &'|'{
            pipe_present = true;
            break;
        }
    }
    
    // now checking all the identifier Checks here
    if pipe_present == false && 
       iden_chars[0] == '^' && 
       iden_chars[identifier.len() - 1] == '$'{
        return (false, "Error: Cannot Use StartsWith and EndsWith Together".to_string());
    }

    return (true, "".to_string());
}

fn get_pattern(iden: &String) -> Process{
    // checking for the OR pattern first here
    let iden_collection: Vec<char> = iden.chars().collect();
    for a_char in iden_collection.iter(){
        if a_char == &'|'{
            return  Process::TWOWORDSEARCH;
        }
    }
    if iden_collection[0] == '^'{
        return Process::STARTSWITH;
    }
    else if iden_collection[iden.len()-1] == '$'{
        return Process::ENDSWITH;
    }
    else {
        return Process::WORDSEARCH;
    }
}

fn main() {
    // getting all the arguments here
    let my_args: Vec<String> = env::args().collect();

    match my_args.len() {
        3 => {
            match check_args(
                &my_args[1],
                &my_args[2]
            ) {
                (val1, val2) => match val1 {
                    false => {
                        let mut val2_error = val2;
                        println!("{}",val2_error.color("red"));
                        return;
                    },
                    _ => {}
                }
            }
            println!("GREPPING DATA...\n")
        },
        _ => {
            invalid_args_message();
            return;
        }
    }

    // creating a operation type Value here
    let oper: Operation = Operation{
        file_data: match fs::read_to_string(&my_args[1]){
            Ok(d) => d,
            Err(er) => panic!("Error: {}",er)
        },
        identifier: my_args[2].clone(),
        process: get_pattern(&my_args[2])
    };

    // now Matching the pattern and getting the result here
    let result: Vec<String> = match oper.process {
        Process::ENDSWITH => endswith_pattern_check(&oper.identifier, &oper.file_data),
        Process::STARTSWITH => startswith_pattern_check(&oper.identifier, &oper.file_data),
        Process::TWOWORDSEARCH => pipe_pattern_check(&oper.identifier, &oper.file_data),
        Process::WORDSEARCH => normal_word_pattern_check(&oper.identifier, &oper.file_data)
    };

    // printing the reuslt here
    for a_reuslt_line in result.iter(){
        println!("{}",a_reuslt_line);
    }
}
