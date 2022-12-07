use regex::Regex;
use crate::utils::read_lines;

pub(crate) fn day_7_1() {
    if let Ok(lines) = read_lines("./data/input_7.txt") {
        let inputs = lines.map(|line| line.unwrap()).collect::<Vec<String>>();
        let tree = parser(inputs.clone());

        let size_dirs = find_size_per_directories(tree.clone());

        let total_size = size_dirs.iter().filter(|x| x <= &&100000).sum::<i32>();
        println!("Total size dir < 100000: {}", total_size);
    }
}

pub(crate) fn day_7_2() {
    if let Ok(lines) = read_lines("./data/input_7.txt") {
        let inputs = lines.map(|line| line.unwrap()).collect::<Vec<String>>();
        let tree = parser(inputs.clone());

        let size_dirs = find_size_per_directories(tree.clone());

        let racine = size_dirs[0];
        let free_space = 70000000 - racine;

        let mut deletable_directories = size_dirs.into_iter().filter(|x| (x + &free_space) >= 30000000).collect::<Vec<i32>>();
        deletable_directories.sort();

        println!("Size of the smallest directory deletable: {}", deletable_directories.first().unwrap());
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct File {
    name: String,
    size: i32,
    parent: String,
    is_directory: bool,
}

fn parser(inputs: Vec<String>) -> Vec<File> {
    let mut files = Vec::new();
    let mut parents: Vec<String> = Vec::from(["racine".to_string()]);
    let re = Regex::new(r"(\d+) ([a-z.]+)").unwrap();

    for command in &inputs[2..] {
        if command.starts_with("$ ls") {
            continue;
        } else if command.starts_with("$ cd ..") {
            parents.pop();
        } else if command.starts_with("$ cd") {
            let dirname = command[5..].to_string();
            parents.push(dirname.clone());
        } else if command.starts_with("dir") {
            files.push(File {
                name: command[4..].to_string(),
                size: 0,
                parent: parents.join("/"),
                is_directory: true,
            });
        } else {
            re.captures_iter(&command).for_each(|cap| {
                files.push(File {
                    name: cap[2].to_string(),
                    size: cap[1].parse::<i32>().unwrap(),
                    parent: parents.join("/"),
                    is_directory: false,
                });
            });
        }
    }

    return files;
}

fn find_size_per_directories(tree: Vec<File>) -> Vec<i32> {
    let mut parents = tree.iter().map(|x| x.parent.clone()).collect::<Vec<String>>();
    parents.dedup();

    let size_dirs: Vec<i32> = parents.iter().map(|parent| {
        let dir_sum = tree.iter()
            .filter(|x| x.parent.starts_with(&*parent))
            .map(|x| x.size).sum::<i32>();

        return dir_sum;
    }).collect();

    return size_dirs;
}
