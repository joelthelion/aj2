use std::fs::File;
use std::io::prelude::*;

fn parse_line(l:&str) -> Result<(f32, &str), ()> {
    let parts = l.split("\t").collect::<Vec<&str>>();
    let weight = parts[0].parse::<f32>();
    match weight {
        Err(_) => Err(()),
        Ok(w) =>  Ok((w, parts[1]))
    }
}

fn parse_file_contents(buf:&str) -> Vec<(f32,&str)> {
    buf.lines()
        .map(parse_line)
        .filter_map(|t| t.ok())
        .collect()
}

fn main()  -> std::io::Result<()>{
    let mut aj_file = File::open("/home/joel/.local/share/autojump/autojump.txt")?;
    let mut contents = String::new();
    aj_file.read_to_string(&mut contents)?;
    let entries = parse_file_contents(contents.as_str());
    for e in &entries {
        println!("{} {}", e.0, e.1);
    }
    println!("#3: {}", entries[3].0);
    Ok(())
}
