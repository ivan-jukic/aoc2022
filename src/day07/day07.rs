use extras::files;
use extras::vec;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

fn main() {
    let input = "src/day07/input";

    let dir_sizes = get_all_dir_sizes(input);

    let pt1 = pt1_solution(&dir_sizes);
    let pt2 = pt2_solution(&dir_sizes);

    println!("day07: Pt1 = {}", pt1);
    println!("day07: Pt2 = {}", pt2);
}

fn pt1_solution(dir_sizes: &HashMap<String, u32>) -> u32 {
    let pt1_sum: HashMap<String, u32> = dir_sizes
        .clone()
        .into_iter()
        .filter(|(_, v)| *v <= 100000)
        .collect();

    pt1_sum.values().sum()
}

fn pt2_solution(dir_sizes: &HashMap<String, u32>) -> u32 {
    let total_size = 70000000;
    let required_size = 30000000;
    let mut available_size = total_size;
    let mut min_available_size = total_size;

    if let Some(reserved_size) = dir_sizes.get(&"/".to_string()) {
        available_size = total_size - *reserved_size;

        for size in dir_sizes.values() {
            let new_available_size = available_size + size;
            if new_available_size < min_available_size && new_available_size >= required_size {
                min_available_size = new_available_size;
            }
        }
    }

    min_available_size - available_size
}

fn get_all_dir_sizes(filename: &str) -> HashMap<String, u32> {
    match vec::uncons(files::read_into_vec_of(filename, |line| Some(line))) {
        Some((_, mut cmds)) => {
            let tree = Rc::new(RefCell::new(FsNode::new(Type::Dir(String::from("/")), 0)));
            decode_fs(&mut cmds, &tree);
            let mut dir_sizes = HashMap::new();
            let start_key = tree.borrow().name.clone().to_string();
            tree.borrow().calc_dir_size(start_key, &mut dir_sizes);
            dir_sizes
        }
        None => panic!("No root cmd!"),
    }
}

///
/// ............................................................................

type Tree<T> = Rc<RefCell<T>>;

#[derive(Clone, Debug, PartialEq)]
struct FsNode {
    name: Type,
    size: u32,
    children: Vec<Tree<FsNode>>,
}

#[derive(Clone, Debug, PartialEq)]
enum Type {
    File(String),
    Dir(String),
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::Dir(dn) => dn.to_string(),
            Type::File(fln) => fln.to_string(),
        }
    }
}

impl FsNode {
    fn new(name: Type, size: u32) -> Self {
        FsNode {
            name,
            size,
            children: vec![],
        }
    }

    fn add_child(&mut self, name: Type, size: u32) -> &Self {
        let children_names: Vec<Type> = self
            .children
            .clone()
            .into_iter()
            .map(|c| c.borrow().name.clone())
            .collect();

        if !children_names.contains(&name) {
            let child = FsNode {
                name,
                size,
                children: vec![],
            };

            self.children.push(Rc::new(RefCell::new(child)));
        }

        self
    }

    fn find_child(&self, needle: Type) -> &Tree<FsNode> {
        self.children
            .iter()
            .find(|c| c.borrow().name == needle)
            .unwrap()
    }

    fn calc_dir_size(&self, key: String, dirs: &mut HashMap<String, u32>) {
        let total: u32 = self
            .children
            .clone()
            .into_iter()
            .map(|c| match &c.borrow().name {
                Type::File(_) => c.borrow().size,
                Type::Dir(name) => {
                    let child_key = format!("{}_{}", key.clone(), name.clone());
                    c.borrow().calc_dir_size(child_key.clone(), dirs);
                    *dirs.get(&child_key).unwrap()
                }
            })
            .sum();

        let self_name = match self.name.clone() {
            Type::Dir(n) => n,
            Type::File(n) => n,
        };

        let prev = dirs.insert(key, total);
        if let Some(v) = prev {
            panic!(
                "ERROR: This key was already available: {} : {} ? {}",
                self_name, v, total
            );
        }
    }

    fn _print(self, level: u32) {
        let _ = (0..level)
            .into_iter()
            .map(|_| {
                print!("...");
            })
            .collect::<()>();
        match self.name {
            Type::File(fname) => {
                println!(". F {} {}", fname, self.size);
            }
            Type::Dir(dname) => {
                println!(". D {}", dname);
            }
        }
        for child in self.children {
            child.borrow().clone()._print(level + 1);
        }
    }
}

fn decode_fs(cmds: &mut Vec<String>, fs: &Tree<FsNode>) {
    loop {
        let local_cmds = cmds.clone();
        match vec::uncons(local_cmds) {
            Some((cmd, other)) => {
                *cmds = other;
                match cmd.split_whitespace().collect::<Vec<&str>>().as_slice() {
                    ["$", "cd", ".."] => break,
                    ["$", "cd", dir] => {
                        let child_name = Type::Dir(dir.to_string());
                        let child_fs = fs.borrow().find_child(child_name).clone();
                        decode_fs(cmds, &child_fs);
                    }
                    ["$", "ls"] => {}
                    ["dir", dir] => {
                        fs.borrow_mut().add_child(Type::Dir(dir.to_string()), 0);
                    }
                    [file_size, file_name] => {
                        let name = file_name.to_string();
                        let size = file_size.parse::<u32>().unwrap();
                        fs.borrow_mut().add_child(Type::File(name), size);
                    }
                    wtf => panic!("LS ERROR: unknown > {:?}", wtf),
                };
            }
            None => break,
        };
    }
}

///
/// Tests
/// ............................................................................

#[cfg(test)]
mod test {
    use crate::get_all_dir_sizes;

    #[test]
    fn test_pt1_solution() {
        let dir_sizes = get_all_dir_sizes("src/day07/input_test");
        let answer = super::pt1_solution(&dir_sizes);
        assert_eq!(answer, 95437);
    }

    #[test]
    fn test_pt2_solution() {
        let dir_sizes = get_all_dir_sizes("src/day07/input_test");
        let answer = super::pt2_solution(&dir_sizes);
        assert_eq!(answer, 24933642);
    }
}
