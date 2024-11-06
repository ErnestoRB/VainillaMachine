use clap::{Args, Parser as CParser, Subcommand};
use std::fs;
use std::io::Read;
use vainilla_machine::parse;
use vainilla_machine::vm;

#[derive(CParser)]
#[command(version = env!("CARGO_PKG_VERSION"), author = "Ernesto Ramírez (https://github.com/ErnestoRB)", about = "CLI for Vainilla Machine", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long)]
    /// Turn on/off processing output
    debug: bool,
    // #[arg(short, long)]
}

#[derive(Subcommand, Clone)]
enum Commands {
    /// Scan files and put output on the same dir with the same name of the file but .lex appended
    Run(RunArgs),
    RunStdin,
    /// Scan files and put output on the same dir with the same name of the file but .lex appended
    Parse(RunArgs),
}

#[derive(Args, Clone)]
struct RunArgs {
    file: String,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run(run_args) => {
            let file_name = &run_args.file;
            if !file_name.ends_with(".vm") {
                eprintln!("Error: The input file must have a .vm extension");
                std::process::exit(1);
            }
            let contents =
                fs::read_to_string(file_name).expect("Something went wrong reading the file");

            let mut parser = parse::Parser::new();
            let instructions = parser.parse_file(&contents);
            let mut vm = vm::VM::new(instructions);

            if cli.debug {
                println!("Ejecutando programa en modo depuración...");
                loop {
                    println!("Seleccione una opción:");
                    println!("1. Ejecutar una instrucción");
                    println!("2. Ejecutar todo el programa");
                    println!("3. Ver contenido de la pila");
                    println!("4. Ver contenido de las variables");
                    println!("5. Salir");

                    let mut choice = String::new();
                    std::io::stdin()
                        .read_line(&mut choice)
                        .expect("Failed to read line");
                    let choice = choice.trim();

                    match choice {
                        "1" => {
                            println!("Instruccion actual: {:?}", vm.current_instruction());
                            vm.step();
                            println!("Instruccion siguiente: {:?}", vm.current_instruction());
                        }
                        "2" => {
                            vm.run();
                        }
                        "3" => {
                            println!("Contenido de la pila:");
                            vm.print_stack();
                        }
                        "4" => {
                            println!("Contenido de las variables:");
                            vm.print_vars();
                        }
                        "5" => {
                            break;
                        }
                        _ => {
                            println!("Opción no válida, por favor intente de nuevo.");
                        }
                    }
                }
            } else {
                println!("Ejecutando programa...");
                vm.run();
            }
        }
        Commands::Parse(run_args) => {
            let file_name = &run_args.file;
            if !file_name.ends_with(".vm") {
                eprintln!("Error: The input file must have a .vm extension");
                std::process::exit(1);
            }
            let contents =
                fs::read_to_string(file_name).expect("Something went wrong reading the file");

            let mut parser = parse::Parser::new();
            let instructions = parser.parse_file(&contents);
            for instr in instructions {
                println!("{:?}", instr);
            }
        }
        Commands::RunStdin => {
            let mut contents = String::new();
            std::io::stdin()
                .read_to_string(&mut contents)
                .expect("Something went wrong reading from stdin");

            let mut parser = parse::Parser::new();
            let instructions = parser.parse_file(&contents);
            let mut vm = vm::VM::new(instructions);

            if cli.debug {
                println!("Ejecutando programa en modo depuración...");
                loop {
                    println!("Seleccione una opción:");
                    println!("1. Ejecutar una instrucción");
                    println!("2. Ejecutar todo el programa");
                    println!("3. Ver contenido de la pila");
                    println!("4. Ver contenido de las variables");
                    println!("5. Salir");

                    let mut choice = String::new();
                    std::io::stdin()
                        .read_line(&mut choice)
                        .expect("Failed to read line");
                    let choice = choice.trim();

                    match choice {
                        "1" => {
                            println!("Instruccion actual: {:?}", vm.current_instruction());
                            vm.step();
                            println!("Instruccion siguiente: {:?}", vm.current_instruction());
                        }
                        "2" => {
                            vm.run();
                        }
                        "3" => {
                            println!("Contenido de la pila:");
                            vm.print_stack();
                        }
                        "4" => {
                            println!("Contenido de las variables:");
                            vm.print_vars();
                        }
                        "5" => {
                            break;
                        }
                        _ => {
                            println!("Opción no válida, por favor intente de nuevo.");
                        }
                    }
                }
            } else {
                println!("Ejecutando programa...");
                vm.run();
            }
        }
    }
}
