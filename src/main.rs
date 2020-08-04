use std::io;
use std::io::*;

/* Crates */
extern crate regex;
use regex::Regex;

/* Consts */
const EXIT_MSG: &str = "exit\n";
const MAIN_ERROR_MSG: &str = "Something went wrong";
const PARSE_ERROR_MSG: &str = "Bad syntax";
const SUM_CHAR: char = '+';
const SUB_CHAR: char = '-';
const MUL_CHAR: char = '*';
const DIV_CHAR: char = '/';

/*
    Main function.
*/
fn main() 
{
    println!( "Welcome");
    println!( "Supported operations: +, -, *, /");
    println!( "Limitations: '*' and '/' cannot be in the same expression");

    loop
    {
        let mut message = input();

        if message.eq( EXIT_MSG )
        {
            println!( "Bye" );
            break;
        }

        message = parse( message ).unwrap();

        if message.len() > 2
        {
            if message.eq( MAIN_ERROR_MSG )
            {
                message = "Bad input".to_string();
            }
            else
            {
                //println!( "{:?}", vectorize( message ) );
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

/*
    User input.
    @return String representing the stdin.
*/
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

/*
    Prepare the user input to be passed to the calculator. Clean spaces and
    does a syntax check.
    @param input user input in String format.
    @return Result enum. In case of Ok, returnes the string input parsed.
            Otherwise, returnes an error message.
*/
fn parse( input : String ) -> std::result::Result<String, String>
{
    let reg = Regex::new( r"[a-zA-Z]" ).unwrap();
    if reg.is_match( &input )
    {
        Err( PARSE_ERROR_MSG.to_string() )
    }
    else
    {
        let reg = Regex::new( r"\s" ).unwrap();
        let parsed_input = reg.replace_all( &input, "" ).to_string();
    
        Ok( parsed_input )
    }
}

/*
    Split the input expression (parsed) into a vector of expressions. Each
    expression is separated from the other by '+' or '-'.
    @param input String parsed.
    @return vector of Strings, representing all the expression.
*/
fn vectorize( input : String ) -> Vec<String>
{
    let mut equ_parts: Vec<String>;
    let mut i: usize;
    let mut first: bool;
    let mut unit: bool;
    let mut last_pos: usize;
    
    equ_parts = Vec::new();
    i = 0;
    first = true;
    unit = true;
    last_pos = 0;

    for ch in input.chars()
    {
        if ch == '+' || ch == '-'
        {
            unit = false;
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
    if unit
    {
        equ_parts.push( input );
    }
    else
    {
        equ_parts.push( input[ last_pos+1..input.len() ].to_string() );
    }

    equ_parts
}

/*
    Does the calculation process.
    @param expression_vector String vector with all the expressions.
    @return String representing the result of the operation.
*/
fn calculate( expression_vector : Vec<String> ) -> String
{
    let mut result: i32;
    let mut i: usize;
    let mut solutions_vector: Vec< String >;

    i = 1;
    solutions_vector = Vec::new();

    if expression_vector.len() == 1
    {
        result = solve_expression( &expression_vector[ 0 ] ).parse().unwrap();
    }
    else
    {
        for j in 0..expression_vector.len()
        {
            solutions_vector.push( solve_expression( &expression_vector[ j ] ) );
        }
    
        result = solutions_vector[ 0 ].parse().unwrap();
    
        loop
        {
            let number: i32;
    
            number = solutions_vector[ i + 1 ].parse().unwrap();
    
            if solutions_vector[ i ].eq( &SUM_CHAR.to_string() )
            {
                result += number;
            }
            else if solutions_vector[ i ].eq( &SUB_CHAR.to_string() )
            {
                result -= number;
            }
    
            i += 2;
    
            if i >= solutions_vector.len()
            {
                break;
            }
        }
    }

    return result.to_string();

}

/*
    Resolve an expression from an expression vector. Supports '*' and '/' 
    signs.
    @param expression the expression to solve.
    @return String representing the expression solved.
*/
fn solve_expression( expression : &str ) -> String
{
    let expression_str : String;
    
    if Regex::new( r"\*" ).unwrap().is_match( expression )
    {
        expression_str = solve_product( expression );
    }
    else if Regex::new( r"/" ).unwrap().is_match( expression )
    {
        expression_str = solve_division( expression );
    }
    else
    {
        expression_str = expression.to_string();
    }

    expression_str
}

/*
    Resolve an expression from an expression vector. Supports only '*' 
    sign.
    @param expression the expression to solve.
    @return String representing the expression solved.
*/
fn solve_product( expression : &str ) -> String
{
    let partitions: Vec< &str >;
    let mut product;
    
    partitions = expression.split( MUL_CHAR ).collect();
    product = 1;

    for i in 0..partitions.len()
    {
        product = product * partitions[ i ].to_string().parse::<i32>().unwrap();
    }
    
    return product.to_string();
}

/*
    Resolve an expression from an expression vector. Supports only '/' 
    sign.
    @param expression the expression to solve.
    @return String representing the expression solved.
*/
fn solve_division( expression : &str ) -> String
{
    let partitions: Vec< &str >;
    let mut product;
    
    partitions = expression.split( DIV_CHAR ).collect();
    product = partitions[ 0 ].to_string().parse::<i32>().unwrap();

    for i in 1..partitions.len()
    {
        product = product / partitions[ i ].to_string().parse::<i32>().unwrap();
    }
    
    return product.to_string();
}