use util::res::Result;

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let contents = util::file::read_to_string(file_path)?;

    println!("Contents: {}", contents);
    Ok(())
}