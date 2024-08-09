use clap::{Parser, Subcommand};

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
}

fn main()
{
    let args = Args::parse();

    match &args.command
    {
        Some(Commands::Init { force }) =>
        {
            if *force
            {
                init::init_repo(true).unwrap();
            }
            else
            {
                init::init_repo(false).unwrap();    
            }
        }
        _ =>
        {
            println!("Not init");
        }
    }
}