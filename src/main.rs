/* Lib */
use simple_calculator::*;

/*
    Main function.
*/
fn main() 
{
    println!( "Welcome");
    println!( "Supported operations: +, -, *, / and **.");
    println!( "Limitations: '*' and '/' cannot be in the same expression");

    'outer_loop: loop
    {
        let mut buffer: String;
        let mut code: simple_calculator::ReturnCodes;
        let mut expressions_vector: Vec< String >;

        'inner_loop: loop
        {
            buffer = String::new();
            
            code = simple_calculator::input( &mut buffer );
            if code == simple_calculator::ReturnCodes::Exit
            {
                println!( "{}", simple_calculator::EXIT_MSG.to_string() );
                break 'outer_loop;
            }

            code = simple_calculator::parse( buffer.clone(), &mut buffer );
            if code != simple_calculator::ReturnCodes::Okey
            {
                println!( "{}", simple_calculator::SYNTAX_ERROR_MSG.to_string() );
                break 'inner_loop;
            }

            expressions_vector = Vec::new();
            simple_calculator::vectorize( &mut buffer, &mut expressions_vector );
            
            code = simple_calculator::calculate( expressions_vector, &mut buffer );
            if code != simple_calculator::ReturnCodes::Okey
            {
                if code == simple_calculator::ReturnCodes::SyntaxErr
                {
                    println!( "{}", simple_calculator::SYNTAX_ERROR_MSG.to_string() );
                }
                else
                {
                    println!( "{}", simple_calculator::MAIN_ERROR_MSG.to_string() );
                }
                break 'inner_loop;
            }

            println!( "{}", &buffer );
        }
    }
}