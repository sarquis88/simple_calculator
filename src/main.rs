use std::io;
use std::io::*;

extern crate regex;
use regex::Regex;

const EXIT_MSG: &str = "exit\n";
const MAX_EQU_PARTS: usize = 2;

fn main() 
{
    println!( "Welcome.\nSupported operations: +, -, *, **, /.");

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
    loop
    {
        let mut input = String::new();
        print!( ">> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line( &mut input ).expect( "error: unable to read user input" );

        if input.chars().next().unwrap() != '\n'
        {
            return input;
        }
    }
}

fn parse( input : String ) -> String
{
    let reg_clean = Regex::new( r"[a-zA-Z]+|\s" ).unwrap();

    let parsed_input = reg_clean.replace_all( &input, "" ).to_string();

    parsed_input
}

fn calculate( input : String ) -> String
{
    let mut sign = String::new();

    let reg_sum = Regex::new( r"(.*?)\+(.*?)" ).unwrap();
    let reg_sub = Regex::new( r"(.*?)\-(.*?)" ).unwrap();
    let reg_mul = Regex::new( r"(.*?)\*(.*?)" ).unwrap();
    let reg_pow = Regex::new( r"(.*?)\*\*(.*?)" ).unwrap();
    let reg_div = Regex::new( r"(.*?)/(.*?)" ).unwrap();

    if reg_sum.is_match( &input )
    {
        sign = "+".to_string();
    }
    else if reg_sub.is_match( &input )
    {
        sign = "-".to_string();
    }
    else if reg_pow.is_match( &input )
    {
        sign = "**".to_string();
    }
    else if reg_mul.is_match( &input )
    {
        sign = "*".to_string();
    }
    else if reg_div.is_match( &input )
    {
        sign = "/".to_string();
    }
    else
    {
        return "NULL".to_string();
    }

    let vector: Vec<&str> = input.split( &sign ).collect();

    if vector.len() > MAX_EQU_PARTS
    {
        return "Too large".to_string();
    }

    let first : i32 = vector    [ 0 ].parse().unwrap();
    let second : i32 = vector   [ 1 ].parse().unwrap();

    if sign.eq( "-" )
    {
        return ( first - second ).to_string();
    }
    else if sign.eq( "+" )
    {
        return ( first + second ).to_string();
    }
    else if sign.eq( "*" )
    {
        return ( first * second ).to_string();
    }
    else if sign.eq( "**" )
    {
        return i32::pow( first, second as u32 ).to_string();
    }
    else if sign.eq( "/" )
    {
        return ( first / second ).to_string();
    }
    else
    {
        return "NULL".to_string();
    }
}