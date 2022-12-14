use std::cell::{RefCell};
use std::rc::Rc;

enum FileType {
    File, Dir
}

struct Node {
    name: String,
    size: usize,
    file_type: FileType,
    parent: Option<Rc<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new() -> Node {
        return Node {
            name: String::new(),
            size: 0,
            file_type: FileType::Dir,
            parent: None,
            children: Vec::new(),
        };
    }

    // pub fn add_child(&mut self, new_node: Rc<RefCell<Node>>) {
    //     self.children.push(new_node);
    // }
    //
    // pub fn add_parent(&mut self, new_node: Option<Rc<RefCell<Node>>>) {
    //     self.parent = new_node;
    // }
    //
    // pub fn print(&self) -> String {
    //     return String::from(&self.name) + " (" + &self.size.to_string() + ")" + if matches!(&self.file_type, FileType::Dir) { ": [" } else { "" }
    //         + &self.children.iter().map(|child| child.borrow().print())
    //         .collect::<Vec<String>>()
    //         .join(", ")
    //         + if matches!(&self.file_type, FileType::Dir) { "]" } else { "" };
    // }
}

pub fn solution(input: String) {
    let root: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node::new()));
    {
        let mut mut_root = root.borrow_mut();
        mut_root.name = String::from("/");
    }
    let mut current: Rc<RefCell<Node>> = Rc::clone(&root);

    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        if words[0] == "$" {
            let command = words[1];
            if command == "cd" {
                let path = words[2];

                if path == "/" {

                } else if path == ".." {
                    let current_clone = Rc::clone(&current);
                    current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
                } else {
                    let child = Rc::new(RefCell::new(Node::new()));
                    {
                        let mut mut_child = child.borrow_mut();
                        mut_child.parent = Some(Rc::clone(&current));
                        mut_child.name = String::from(path);
                        mut_child.file_type = FileType::Dir
                    }
                    current.borrow_mut().children.push(Rc::clone(&child));
                    current = child;
                }
            }
        } else {
            if words[0] != "dir" {
                let size: usize = words[0].parse().unwrap();
                let child: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node::new()));
                current.borrow_mut().children.push(Rc::clone(&child));
                {
                    let mut mut_child = child.borrow_mut();
                    mut_child.parent = Some(Rc::clone(&current));
                    mut_child.name = String::from(words[1]);
                    mut_child.size = size;
                    mut_child.file_type = FileType::File;
                    current.borrow_mut().size += size;
                }
                update_size(&current.borrow_mut().parent, size);

            }
        }
    }

    let r = &root.borrow_mut();
    let unused_space = 70000000 - r.size;
    let free_space = 30000000 - unused_space;
    let size = calculate_total_under_n(&r.children, 100000);
    let mut sizes = calculate_smallest_to_delete(&r.children, free_space);
    sizes.sort();
    println!("Total size of directories under 100,000: {}", size);
    println!("Total size of smallest directory to delete: {}", sizes[0]);
}

fn update_size(node: &Option<Rc<RefCell<Node>>>, size: usize) {
    match node {
        None => return,
        Some(n) => {
            n.borrow_mut().size += size;
            update_size(&n.borrow_mut().parent, size);
        }
    }
}

fn calculate_total_under_n(children: &Vec<Rc<RefCell<Node>>>, size: usize) -> usize {
    let mut total_size: usize = 0;
    for child in children {
        let c = child.borrow();
        if matches!(c.file_type, FileType::Dir) && c.size <= size {
            total_size += c.size;
        }
        total_size += calculate_total_under_n(&child.borrow().children, size);
    }
    total_size
}

fn calculate_smallest_to_delete(children: &Vec<Rc<RefCell<Node>>>, size: usize) -> Vec<usize> {
    let mut sizes: Vec<usize> = Vec::new();
    for child in children {
        let c = child.borrow();
        if matches!(c.file_type, FileType::Dir) && c.size >= size {
            sizes.push(c.size);
        }
        sizes.append(&mut calculate_smallest_to_delete(&child.borrow().children, size));
    }
    sizes
}