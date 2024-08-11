use std::io::Result;
use std::path::Path;

use clap::{Parser, Subcommand};

mod add;
mod commit;
mod init;

#[derive(Parser)]
struct Args
{
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands
{
    /// Initialize a new Cog repository
    Init {
        /// Force initialization
        #[arg(short, long)]
        force: bool,
    },

    /// Stage files for committing
    Add {
        filename: String,
    },

    /// Commit changes
    Commit {
        /// Commit message
        #[arg(short, long)]
        message: String,
    }
}

fn main() -> Result<()>
{
    let args = Args::parse();

    match &args.command
    {
        Some(Commands::Init { force }) =>
        {
            if *force
            {
                init::init_repo(true)?;
            }
            else
            {
                init::init_repo(false)?;    
            }

            return Ok(())
        },

        Some(Commands::Add { filename }) =>
        {
            let obj_path = Path::new(".cog/objects");

            if obj_path.exists()
            {
                add::add_file(&filename)?;
                return Ok(())
            }
            println!("Object path does not exist! Most likely you haven't initialized a Cog repo yet. Try running cog init to get started :)");

            return Ok(())
        },

        Some(Commands::Commit { message }) =>
        {
            todo!()
        }
        _ =>
        {
            println!("Not a valid command! Try running cog -h");

            return Ok(())
        }
    }
}