use std::io;
use std::io::*;

extern crate regex;
use regex::Regex;

const EXIT_MSG: &str = "exit\n";
const ERROR_MSG: &str = "err";

const SUM_CHAR: char = '+';
const SUB_CHAR: char = '-';
//const MUL_CHAR: char = '*';
//const DIV_CHAR: char = '/';

fn main() 
{
    println!( "Welcome.\nSupported operations: +, -.");

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
            if message.eq( ERROR_MSG )
            {
                message = "Bad input".to_string();
            }
            else
            {
                message = calculate( vectorize( message ) );
            }
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
    let reg = Regex::new( r"\s" ).unwrap();
    let parsed_input = reg.replace_all( &input, "" ).to_string();

    let reg = Regex::new( r"\-|\+" ).unwrap();
    let sign_cantity = reg.find_iter( &input ).count();

    let reg = Regex::new( r"[0-9]+" ).unwrap();
    let number_cantity = reg.find_iter( &input ).count();

    if number_cantity == sign_cantity + 1
    {
        return parsed_input;
    }
    else
    {
        return ERROR_MSG.to_string();
    }
}

fn vectorize( input : String ) -> Vec<String>
{
    let mut equ_parts: Vec<String> = Vec::new();
    let mut i: usize = 0;
    let mut first = true;
    let mut last_pos: usize = 0;

    for ch in input.chars()
    {
        if (ch as i8) < 48 || (ch as i8) > 57
        {
            if first
            {
                equ_parts.push( input[ 0..i ].to_string() );
                first = false;
            }
            else
            {
                equ_parts.push( input[ last_pos+1..i ].to_string() );
            }
            equ_parts.push( ch.to_string() );
            last_pos = i;
        }
        i += 1;
    }
    equ_parts.push( input[ last_pos+1..input.len() ].to_string() );

    equ_parts
}

fn calculate( input_vector : Vec<String> ) -> String
{
    let mut result: i32 = 0;
    let mut i: usize = 1;

    result = input_vector[ 0 ].parse().unwrap();
    loop
    {
        let number: i32 = input_vector[ i + 1 ].parse().unwrap();

        if input_vector[ i ].eq( &SUM_CHAR.to_string() )
        {
            result += number;
        }
        else if input_vector[ i ].eq( &SUB_CHAR.to_string() )
        {
            result -= number;
        }

        i += 2;

        if i >= input_vector.len()
        {
            break;
        }
    }

    return result.to_string();

}