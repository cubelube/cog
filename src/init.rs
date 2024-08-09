use std::{
    path::Path,

    fs,

    io::{Result, stdin, stdout, Write}
};

fn cog_init(cog_path: &Path, remove: bool) -> Result<()>
{
    if remove
    {
        fs::remove_dir_all(cog_path)?;
    }

    fs::create_dir(cog_path)?;
    fs::create_dir(cog_path.join("revs"))?;

    Ok(())
}

pub fn init_repo(force: bool) -> Result<()>
{
    let cog_path = Path::new(".cog");

    if cog_path.exists()
    {
        if force
        {
            cog_init(cog_path, true)?;
            println!("Cog reinitialized");
            return Ok(())
        }
        else
        {
            let mut input = String::new();

            print!("It seems that a Cog repository has already been initialized. Reinitialize anyway? [y/N] ");
            stdout().flush().unwrap();
            stdin().read_line(&mut input).unwrap();

            if input.trim().to_string().to_lowercase() != "y" && input.trim().to_string().to_lowercase() != "yes"
            {
                println!("Aborting...");
                return Ok(())
            }
            else
            {
                cog_init(cog_path, true)?;
                println!("Cog reinitialized");
                return Ok(())
            }
        }
    }

    cog_init(cog_path, false)?;

    println!("Cog repo initialized");

    Ok(())
}