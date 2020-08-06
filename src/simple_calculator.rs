/* Crates */
extern crate regex;
use std::io;
use std::io::*;
use regex::Regex;

/* Consts messages */
pub const EXIT_INPUT: &str = "exit\n";
pub const HELP_ARG: &str = "--help";
pub const H_ARG: &str = "-h";
pub const EXIT_MSG: &str = "Bye";
pub const MAIN_ERROR_MSG: &str = "Something went wrong";
pub const SYNTAX_ERROR_MSG: &str = "Bad syntax";
pub const HELP_ARG_MSG: &str = "Supported operations: '+', '-', '*', '/', '**'.\nLimitations: cannot use two (differents) of '*', '/' or '**' between '+' or '-' signs.";
pub const UNKNOWN_ARG_MSG: &str = "Unrecognized option";  /* consider adding ": <unknown_arg>" */
pub const ARGS_LIST_MSG: &str = "Recognized options:\n  -h  --help  prints usage and exit";

/* Consts codes */
pub const SUM_CHAR: char = '+';
pub const SUB_CHAR: char = '-';
pub const MUL_CHAR: char = '*';
pub const DIV_CHAR: char = '/';
pub const POW_STR: &str = "**";

/* Enums */
#[derive(PartialEq, Eq)]
pub enum ReturnCodes
{
    Okey = 0,
    MainErr,
    SyntaxErr,
    Exit
}

/// User input. \
/// @param  buffer for store the input. \
/// @return ReturnCode depending the succes of the operation.
pub fn input( buffer: &mut String ) -> ReturnCodes
{   
    loop
    {
        print!( ">> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line( buffer ).unwrap();

        if buffer.chars().next().unwrap() != '\n'
        {
            break;
        }
    }

    if buffer.to_string().eq( EXIT_INPUT )
    {
        return ReturnCodes::Exit;
    }

    return ReturnCodes::Okey;
}

/// Prepare the user input to be passed to the calculator. Clean spaces and 
/// does a syntax check. \
/// @param  input user input in String format. \
/// @param  String buffer for storing the parsed String. \
/// @return ReturnCode depending the succes of the operation.
pub fn parse( input: String, buffer: &mut String ) -> ReturnCodes
{
    let reg = Regex::new( r"[a-zA-Z]" ).unwrap();
    if reg.is_match( &input )
    {
        return ReturnCodes::SyntaxErr;
    }
    else
    {
        let reg = Regex::new( r"\s" ).unwrap();
        *buffer = reg.replace_all( &input, "" ).to_string();

        if buffer.len() > 2
        {
            if deep_check( buffer.to_string() )
            {
                ReturnCodes::Okey
            }
            else
            {
                ReturnCodes::SyntaxErr
            }
        }
        else
        {
            return ReturnCodes::SyntaxErr;
        }
    }
}

/// Char-to-char check. Ensures that the String has only numbers and the
/// operations signs. \
/// @param  input parsed input to check. \
/// @return true for a correct syntax 
///         false otherwise
fn deep_check( input: String ) -> bool
{
    let input_vector: Vec<char>;
    
    input_vector = input.chars().collect();

    for ch in input_vector
    {
        if (ch as u8) < 48 || (ch as u8) > 57
        {
            if ch != '+' && ch != '-' && ch != '/' && ch != '*'
            {
                return false;
            }
        }
    }

    return true;
}

/// Split the input expression (parsed) into a vector of expressions. Each
/// expression is separated from the other by '+' or '-'. \
/// @param  expression_str input String parsed. \
/// @param  buffer_vector buffer for storing the vectorized expression. \
/// @return ReturnCode depending the succes of the operation.
pub fn vectorize( expression_str : &mut String, buffer_vector: &mut Vec< String > ) -> ReturnCodes
{
    let mut i: usize;
    let mut first: bool;
    let mut unit: bool;
    let mut last_pos: usize;
    
    i = 0;
    first = true;
    unit = true;
    last_pos = 0;

    for ch in expression_str.chars()
    {
        if ch == '+' || ch == '-'
        {
            unit = false;
            if first
            {
                buffer_vector.push( expression_str[ 0..i ].to_string() );
                first = false;
            }
            else
            {
                buffer_vector.push( expression_str[ last_pos+1..i ].to_string() );
            }
            buffer_vector.push( ch.to_string() );
            last_pos = i;
        }
        i += 1;
    }
    if unit
    {
        buffer_vector.push( expression_str.to_string() );
    }
    else
    {
        buffer_vector.push( expression_str[ last_pos+1..expression_str.len() ].to_string() );
    }

    ReturnCodes::Okey
}

/// Does the calculation process. \
/// @param  expression_vector String vector with all the expressions. \
/// @param  buffer for storing the result. \
/// @return ReturnCode depending the succes of the operation.
pub fn calculate( expression_vector: Vec<String>, buffer: &mut String) -> ReturnCodes
{
    let mut i: usize;
    let mut code: ReturnCodes;

    i = 1;

    if expression_vector.len() == 1
    {
        code = solve_expression( &expression_vector[ 0 ], buffer );
        if code != ReturnCodes::Okey
        {
            return code;
        }
        else
        {
            return ReturnCodes::Okey;
        }
    }
    else
    {
        let mut result: i32;
        let mut solutions_vector: Vec< String >;

        solutions_vector = Vec::new();

        for j in 0..expression_vector.len()
        {
            code = solve_expression( &expression_vector[ j ], buffer );
            if code != ReturnCodes::Okey
            {
                return code;
            }
            else
            {
                solutions_vector.push( buffer.to_string() );
            }
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

        *buffer = result.to_string();
        return ReturnCodes::Okey;
    }
}

/// Resolve an expression from an expression vector. Supports '*', '**' and '/' 
/// signs. \
/// @param  expression the expression to solve. \
/// @param  buffer for store the expression solved. \
/// @return ReturnCode depending the succes of the operation. 
fn solve_expression( expression: &str, buffer: &mut String ) -> ReturnCodes
{
    
    if Regex::new( r"\*\*" ).unwrap().is_match( expression )
    {
        solve_pow( expression, buffer )
    }
    else if Regex::new( r"\*" ).unwrap().is_match( expression )
    {
        solve_product( expression, buffer )
    }
    else if Regex::new( r"/" ).unwrap().is_match( expression )
    {
        solve_division( expression, buffer )
    }
    else
    {
        *buffer = expression.to_string();
        ReturnCodes::Okey
    }
}

/// Resolve an expression from an expression vector. Supports only '*' 
/// sign. \
/// @param  expression the expression to solve. \
/// @param  buffer for store the expression solved. \
/// @return ReturnCode depending the succes of the operation.
fn solve_product( expression: &str, buffer: &mut String ) -> ReturnCodes
{
    let partitions: Vec< &str >;
    let mut product;
    
    partitions = expression.split( MUL_CHAR ).collect();
    product = 1;

    for i in 0..partitions.len()
    {
        product = product * partitions[ i ].to_string().parse::<i32>().unwrap();
    }
    
    *buffer = product.to_string();

    return ReturnCodes::Okey;
}

/// Resolve an expression from an expression vector. Supports only '/' 
/// sign. \
/// @param  expression the expression to solve. \
/// @param  buffer for store the expression solved. \
/// @return ReturnCode depending the succes of the operation.
fn solve_division( expression: &str, buffer: &mut String ) -> ReturnCodes
{
    let partitions: Vec< &str >;
    let mut product;
    
    partitions = expression.split( DIV_CHAR ).collect();
    product = partitions[ 0 ].to_string().parse::<i32>().unwrap();

    for i in 1..partitions.len()
    {
        product = product / partitions[ i ].to_string().parse::<i32>().unwrap();
    }
    
    *buffer = product.to_string();

    return ReturnCodes::Okey;
}

/// Resolve an expression from an expression vector. Supports only '**' signs. \
/// @param  expression the expression to solve. \
/// @param  buffer for store the expression solved. \
/// @return ReturnCode depending the succes of the operation.
fn solve_pow( expression: &str, buffer: &mut String ) -> ReturnCodes
{
    let partitions: Vec< &str >;
    let base: i32;
    let exp: u32;
    
    partitions = expression.split( POW_STR ).collect();
    
    if  partitions.len() != 2       ||
        partitions[ 0 ].is_empty()  || 
        partitions[ 1 ].is_empty()
    {
        return ReturnCodes::SyntaxErr;
    }
    else
    {
        base = partitions[ 0 ].to_string().parse().unwrap();
        exp = partitions[ 1 ].to_string().parse().unwrap();

        *buffer = base.pow( exp ).to_string();

        return ReturnCodes::Okey;
    }
}