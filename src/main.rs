use std::io;
use std::io::*;

extern crate regex;
use regex::Regex;

const EXIT_MSG: &str = "exit\n";
const MAX_EQU_PARTS: usize = 2;

fn main() 
{
    loop
    {
        let mut message = input();

        if message.eq( EXIT_MSG )
        {
            println!( "Bye" );
            break;
        }

        message = parse( message );

        if message.len() > 2
        {
            message = calculate( message );
        }
        else
        {
            message = "Bad input".to_string();
        }

        println!( "{}", &message );
    }
}

fn input() -> String
{   
    let mut input = String::new();

    print!( ">> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line( &mut input ).expect( "error: unable to read user input" );

    input
}

fn parse( input : String ) -> String
{
    let reg_clean = Regex::new( r"[a-zA-Z]+|\s" ).unwrap();

    let parsed_input = reg_clean.replace_all( &input, "" ).to_string();

    parsed_input
}

fn calculate( input : String ) -> String
{
    let sign : char;

    let reg_sum = Regex::new( r"(.*?)\+(.*?)" ).unwrap();
    let reg_sub = Regex::new( r"(.*?)\-(.*?)" ).unwrap();
    if reg_sum.is_match( &input )
    {
        sign = '+';
    }
    else if reg_sub.is_match( &input )
    {
        sign = '-';
    }
    else
    {
        return "NULL".to_string();
    }

    let vector: Vec<&str> = input.split( sign ).collect();

    if vector.len() > MAX_EQU_PARTS
    {
        return "Too large".to_string();
    }

    let first : i32 = vector    [ 0 ].parse().unwrap();
    let second : i32 = vector   [ 1 ].parse().unwrap();

    if sign == '-'
    {
        return ( first - second ).to_string();
    }
    else if sign == '+'
    {
        return ( first + second ).to_string();
    }
    else
    {
        return "NULL".to_string();
    }
}