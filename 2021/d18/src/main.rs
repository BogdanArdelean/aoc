use std::cell::{Ref, RefCell};
use std::collections::LinkedList;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::rc::{Rc, Weak};
use crate::Content::Leaf;
use crate::Direction::Left;
use anyhow::Result;

#[derive(Debug, Clone)]
struct Node {
    parent: Option<Weak<RefCell<Node>>>,
    content: Content,
}

#[derive(Debug, Clone)]
enum Content {
    Leaf(i32),
    List(Rc<RefCell<Node>>, Rc<RefCell<Node>>)
}

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right
}

type DList = LinkedList<Direction>;

impl Node {
    fn new() -> Self {
        Node {
            parent: None,
            content: Leaf(0),
        }
    }

    fn create(parent: Option<Weak<RefCell<Node>>>, content: Box<dyn Fn(Option<Weak<RefCell<Node>>>) -> Content>) -> Rc<RefCell<Node>> {
        let mut node = Rc::new(RefCell::new(Node::new()));
        node.borrow_mut().parent = parent;
        node.borrow_mut().content = content(Some(Rc::downgrade(&node)));

        node
    }

    fn create_template<F>(parent: Option<Weak<RefCell<Node>>>, content: F)  -> Rc<RefCell<Node>>
        where F: Fn(Option<Weak<RefCell<Node>>>) -> Content {
        let mut node = Rc::new(RefCell::new(Node::new()));
        node.borrow_mut().parent = parent;
        node.borrow_mut().content = content(Some(Rc::downgrade(&node)));

        node
    }

    fn explode(slf: Rc<RefCell<Node>>, depth: i32, stack: &mut DList) -> bool {
        if depth >= 4 {
            let content = slf.borrow().get_leaf_pair().clone();
            if let Some((l1, l2)) = content {
                Self::modify_left(slf.borrow().parent.clone().unwrap(), l1, stack.clone());
                Self::modify_right(slf.borrow().parent.clone().unwrap(), l2, stack.clone());
                slf.borrow_mut().content = Content::Leaf(0);
                return true;
            }
        }
        let content = slf.borrow().content.clone();
        match content {
            Content::Leaf(i) => {
                return false;
            }
            Content::List(left, right) => {
                stack.push_back(Direction::Left);
                let left_node = left.clone();
                let res = Node::explode(left_node,depth + 1, stack);
                stack.pop_back();
                if res {
                    return res;
                }
                stack.push_back(Direction::Right);
                let right_node = right.clone();
                let res = Node::explode(right_node, depth + 1, stack);
                stack.pop_back();
                if res {
                    return res;
                }
            }
        }

        return false;
    }

    fn modify_left(mut parent: Weak<RefCell<Node>>, value: i32, mut stack: DList) {
        while let Some(Left) = stack.back() {
            stack.pop_back();
            let p1 = parent.upgrade().unwrap().borrow().parent.clone();
            if p1.is_some() {
                parent = p1.unwrap();
            }
        }

        if stack.is_empty() {
            return;
        }

        let mut parent = parent.upgrade().unwrap();
        if let Content::List(left, ..) = parent.clone().borrow().content.clone() {
            parent = left.clone();
        } else {
            return;
        }

        while let Content::List(_, right) = parent.clone().borrow().content.clone() {
            let right = right.clone();
            parent = right;
        }

        let lf = parent.clone().borrow().content.clone();
        if let Content::Leaf(x) = lf {
            parent.borrow_mut().content = Content::Leaf(x + value);
        }
    }

    fn modify_right(mut parent: Weak<RefCell<Node>>, value: i32, mut stack: DList) {
        while let Some(Direction::Right) = stack.back() {
            stack.pop_back();
            let p1 = parent.upgrade().unwrap().borrow().parent.clone();
            if p1.is_some() {
                parent = p1.unwrap();
            }
        }

        if stack.is_empty() {
            return;
        }

        let mut parent = parent.upgrade().unwrap();
        if let Content::List(_, right) = parent.clone().borrow().content.clone() {
            parent = right.clone();
        } else {
            return;
        }

        while let Content::List(left, _) = parent.clone().borrow().content.clone() {
            parent = left.clone();
        }
        let lf = parent.clone().borrow().content.clone();

        if let Content::Leaf(x) = lf {
            parent.borrow_mut().content = Content::Leaf(x + value);
        }
    }

    fn get_leaf_pair(&self) -> Option<(i32, i32)> {
        if let Content::List(c1, c2) = &self.content {
            let lit1 = &c1.borrow().content;
            let lit2 = &c2.borrow().content;
            match (lit1, lit2) {
                (Content::Leaf(l1), Content::Leaf(l2)) => Some((*l1, *l2)),
                (_, _) => None
            }
        } else {
            None
        }
    }

    fn split(slf: Rc<RefCell<Node>>) -> bool {
        let content = slf.borrow().content.clone();

        return match content {
            Content::Leaf(i) => {
                if i >= 10 {
                    let div = i as f64 / 2.0;
                    let parent = Some(Rc::downgrade(&slf));
                    slf.borrow_mut().content = Content::List(
                        Node::create_template(parent.clone(), |_| {
                            Content::Leaf(div.floor() as i32)
                        }),
                        Node::create_template(parent, |_| {
                            Content::Leaf(div.ceil() as i32)
                        })
                    );
                    true
                } else {
                    false
                }
            }
            Content::List(left, right) => {
                let res = Node::split(left);
                if res {
                    return res;
                }
                let res = Node::split(right);
                if res {
                    return res;
                }

                false
            }
        }
    }

    fn reduce(slf: Rc<RefCell<Node>>) {
        loop {
            if Node::explode(slf.clone(), 0, &mut DList::new()) {
                // println!("Explode");
                // Node::print(slf.clone());
                continue;
            }

            if Node::split(slf.clone()) {
                // println!("Split");
                // Node::print(slf.clone());
                continue;
            }

            break;
        }
    }

    fn add(a: Rc<RefCell<Node>>, b: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        Node::create_template(None, |p| {
            a.borrow_mut().parent = p.clone();
            b.borrow_mut().parent = p.clone();

            Content::List(a.clone(), b.clone())
        })
    }

    fn parse(parent: Option<Weak<RefCell<Node>>>, s: &str) -> Rc<RefCell<Node>> {
        if s.len() == 1 {
            return Node::create_template(parent, |_| {
                Content::Leaf(s.parse().unwrap())
            });
        }

        let mut count = 0;
        let mut split = 0;

        for (idx, chr) in s.chars().enumerate() {
            match chr {
                '[' => count+=1,
                ']' => count-=1,
                ',' => if count == 1 {
                    split = idx;
                    break;
                }
                _ => {}
            }
        }

        Node::create_template(parent, |p| {
            Content::List(
                Node::parse(p.clone(), &s[1..split]),
                Node::parse(p, &s[split+1..s.len()-1])
            )
        })
    }

    fn print_r(slf: Rc<RefCell<Node>>) {
        let content = slf.borrow().content.clone();
        match content {
            Leaf(i) => {
                print!("{}",i)
            }
            Content::List(l, r) => {
                print!("[");
                Node::print_r(l.clone());
                print!(",");
                Node::print_r(r.clone());
                print!("]");
            }
        }
    }

    fn magnitude(slf: Rc<RefCell<Node>>) -> i32 {
        let content = slf.borrow().content.clone();
        match content {
            Leaf(i) => {
                i
            }
            Content::List(l, r) => {
                3 * Node::magnitude(l) + 2 * Node::magnitude(r)
            }
        }
    }

    fn print(slf: Rc<RefCell<Node>>) {
        Node::print_r(slf);
        println!();
    }

    fn deep_copy_i(parent: Option<Weak<RefCell<Node>>>, slf: NodePtr) -> NodePtr {
        let n = Rc::new(RefCell::new(Node::new()));
        n.borrow_mut().parent = parent;

        let content = slf.borrow().content.clone();
        n.borrow_mut().content = match content {
            Leaf(i) => Content::Leaf(i),
            Content::List(l, r) => Content::List(
                Node::deep_copy_i(Some(Rc::downgrade(&n)), l),
                Node::deep_copy_i(Some(Rc::downgrade(&n)), r)
            )
        };

        n
    }

    fn deep_copy(slf: Rc<RefCell<Node>>) -> NodePtr {
        Node::deep_copy_i(None, slf)
    }
}

type NodePtr = Rc<RefCell<Node>>;

fn parse(path: &Path) -> Result<Vec<NodePtr>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut vc = Vec::new();
    for line in reader.lines() {
        let line = line?;
        vc.push(Node::parse(None, &line));
    }

    Ok(vc)
}

fn part2(v: &Vec<NodePtr>) -> i32 {
    let mut max = 0;

    for i in 0..v.len() {
        for j in 0..v.len() {
            if i == j {
                continue
            }
            let n1 = Node::deep_copy(v[i].clone());
            let n2 = Node::deep_copy(v[j].clone());

            let sum = Node::add(n1, n2);

            Node::reduce(sum.clone());
            max = max.max(Node::magnitude(sum));
        }
    }

    max
}

fn main() -> Result<()> {
    let vc = parse(&Path::new("input.txt"))?;
    let p2 = part2(&vc);

    let sum = vc.into_iter().reduce(|acc, e| {
        let n = Node::add(acc.clone(), e.clone());
        Node::reduce(n.clone());
        n
    }).unwrap();
    Node::print(sum.clone());
    let magn = Node::magnitude(sum);
    println!("Part 1 {}", magn);
    println!("Part 2 {}", p2);
    Ok(())
}
