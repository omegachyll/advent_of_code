use std::{collections::HashMap, error::Error, fs, time::Instant};
use tracing::info;

#[derive(Debug, PartialEq, Eq)]
struct Folder<'a> {
    parent: &'a str,
    child_dirs: Vec<&'a str>,
    files: Vec<(&'a str, u32)>,
}

impl<'a> Folder<'a> {
    fn new(parent: &str) -> Folder {
        Folder {
            parent,
            child_dirs: Vec::new(),
            files: Vec::new(),
        }
    }
    fn add_child_dir(&mut self, dir: &'a str) {
        info!("{}", dir);
        if !self.child_dirs.contains(&dir) {
            info!("{}- does not exist", dir);
            self.child_dirs.push(dir);
            info!(?self.child_dirs);
        }
    }
    fn add_file(&mut self, name: &'a str, size: u32) {
        let file = (name, size);
        if !self.files.contains(&file) {
            self.files.push(file);
        }
    }
    fn cals_size(self) -> u32 {
        let file_size = self.files.iter().map(|(_, size)| size).sum::<u32>();
        let mut folder_sizes = 0
        if self.child_dir.len() >0 {
        self.child_dir.iter().map(|dir|{
        })

        }
        todo!()
    }
}
fn parse(input: &str) -> HashMap<&str, Folder> {
    let mut system: HashMap<&str, Folder> = HashMap::default();
    let mut cur_dir = "/";
    system.insert("/", Folder::new("."));
    for line in input.lines() {
        info!(line);
        let items: Vec<&str> = line.split(" ").collect();
        if items[0] == "$" {
            if items[1] == "cd" {
                info!(cur_dir);
                if items[2] == ".." {
                    cur_dir = system.get(cur_dir).unwrap().parent;
                    info!(cur_dir);
                } else {
                    if !system.contains_key(items[2]) {
                        let folder = Folder::new(cur_dir);
                        system.insert(items[2], folder);
                    }
                    cur_dir = items[2];
                    info!(cur_dir);
                }
            } else {
                // no action is ls
                continue;
            }
            //its a cmd
        } else if items[0] == "dir" {
            info!("{}", items[0]);
            info!("{}", items[1]);
            system
                .get_mut(cur_dir)
                .expect("should exist")
                .add_child_dir(items[1]);
        } else {
            let (name, size) = (items[1], items[0].parse::<u32>().unwrap());
            info!("{}", name);
            info!("{}", size);

            system
                .get_mut(cur_dir)
                .expect("should exist")
                .add_file(name, size);
        }
    }
    system
}

fn process(input: &str) -> Result<String, Box<dyn Error>> {
    let system = parse(input);
    info!(?system);
    let result = "";
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), Box<dyn Error>> {
        tracing_subscriber::fmt::init();
        let input = fs::read_to_string("test_input.txt").expect("should be string");
        assert_eq!("13", process(input.as_str())?);
        Ok(())
    }
}

#[tracing::instrument]
fn main() {
    tracing_subscriber::fmt::init();
    let now = Instant::now();
    let input = fs::read_to_string("input.txt").expect("should be string");
    println!("{:?}", process(input.as_str()).expect("should be a string"));
    let elapsed = now.elapsed();
    println!("Elapsed : {:.2?}", elapsed);
}
