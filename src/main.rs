use std::{
    io::{self, Write},
    process::exit,
};
use anyhow::{self, Error};

enum MetaCommandResult {
    Success,
    UnrecognizedCommand,
}

fn print_prompt() {
    io::stdout().write_all("db > ".as_bytes()).unwrap();
    io::stdout().flush().unwrap();
}

fn exec_meta_command(command: &str) -> MetaCommandResult {
    if command == ".exit" {
        exit(0);
    } else {
        return MetaCommandResult::UnrecognizedCommand;
    }
}

#[derive(Debug)]
enum StatementType {
    Unexpected,
    Insert,
    Select,
}

#[derive(Debug)]
struct Statement {
    statement_type: StatementType,
}

impl Statement {
    pub fn new(stmt_str: &str) -> anyhow::Result<Self> {
        let mut statement = Self {
            statement_type: StatementType::Unexpected,
        };
        let mut stmt_str = stmt_str.to_string();
        stmt_str = stmt_str.trim().to_lowercase();

        if stmt_str.starts_with("insert") {
            statement.statement_type = StatementType::Insert;
        }
        if stmt_str.starts_with("select") {
            statement.statement_type = StatementType::Select;
        }

        if let StatementType::Unexpected = statement.statement_type {
            return Err(Error::msg("Unexpected statement type"))
        }

        Ok(statement)
    }

    pub fn exec_statment(&self) -> anyhow::Result<()> {
        match self.statement_type {
            StatementType::Insert => {
                println!("This is where we would do an insert.");
            },
            StatementType::Select => {
                println!("This is where we would do a select.");
            },
            _ => (),
        }
        Ok(())
    }
}

fn main() {
    loop {
        print_prompt();

        let mut input_buf = "".to_string();
        io::stdin().read_line(&mut input_buf).unwrap();

        if input_buf.starts_with(".") {
            match exec_meta_command(&input_buf) {
                MetaCommandResult::Success => (),
                MetaCommandResult::UnrecognizedCommand => {
                    println!("Unrecognized command '{}'.", input_buf.trim_end())
                }
            };
            continue;
        }
        
        let stmt = match Statement::new(&input_buf) {
            Ok(statement) => statement,
            Err(_) => {
                println!("Unrecognized keyword at start of '{}'.", input_buf.trim_end());
                continue;
            }
        };

        stmt.exec_statment().unwrap();
        println!("Excuted.")
    }
}
