/// Lib imported.
use std::env;
extern crate ctrlc;
use std::process;

/// Main function.
fn main() 
{
    let args: Vec<String>;
    
    args = env::args().collect();
    if args.len() > 1
    {
        if  args[ 1 ].eq( simple_calculator::HELP_ARG ) |
            args[ 1 ].eq( simple_calculator::H_ARG )
        {
            println!( "{}", simple_calculator::HELP_ARG_MSG );
        }
        else
        {
            println!( "{}: {}", simple_calculator::UNKNOWN_ARG_MSG, args[ 1 ] );
            println!( "{}", simple_calculator::ARGS_LIST_MSG );
        }
        return;
    }

    ctrlc::set_handler( move || {
        println!( "" );
        exit_routine();
        process::exit( 0 );
    })
    .unwrap();

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
                exit_routine();
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

fn exit_routine()
{
    println!( "{}", simple_calculator::EXIT_MSG.to_string() );
}