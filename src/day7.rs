use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

struct BagLink {
    qty: u64,
    link: Rc<RefCell<Bag>>,
}

struct Bag {
    name: String,
    held_bags: Vec<BagLink>,
    containing_bag: Vec<BagLink>,
}

impl Bag {
    fn new(name: &str) -> Self {
        Bag {
            name: name.to_string(),
            held_bags: vec![],
            containing_bag: vec![],
        }
    }

    fn add_held_bag(&mut self, qty: u64, link: Rc<RefCell<Bag>>) {
        self.held_bags.push( BagLink {qty, link});
    }

    fn add_containing_bag(&mut self, qty: u64, link: Rc<RefCell<Bag>>) {
        self.containing_bag.push( BagLink {qty, link});
    }

    fn unique_bag_combos_inner(&self, list: Rc<RefCell<HashSet<String>>>) {
        for parent in &self.containing_bag {
            let parent_bag = parent.link.borrow();
            {
                let mut mut_list = list.borrow_mut();
                mut_list.insert(parent_bag.name.clone());
            }
            parent_bag.unique_bag_combos_inner(list.clone());
        }
    }

    fn unique_bag_combos(&self) -> i64 {
        let set = Rc::new(RefCell::new(HashSet::new()));
        self.unique_bag_combos_inner(set.clone());
        let set = set.borrow();
        set.len() as i64
    }

    fn total_internal_bags(&self) -> i64 {
        let mut sum = 0;
        for inner in &self.held_bags {
            let inner_bag = inner.link.borrow();
            sum += (inner_bag.total_internal_bags() + 1) * (inner.qty as i64);
        }
        sum
    }
}

struct BagChain {
    bags: HashMap<String, Rc<RefCell<Bag>>>,
}

impl BagChain {
    fn new() -> Self {
        BagChain { bags: HashMap::new() }
    }

    fn get_or_create_bag(&mut self, name: &str) -> Rc<RefCell<Bag>> {
        if let Some(b) = self.bags.get(name) {
            b.clone()
        } else {
            let b = Rc::new(RefCell::new(Bag::new(name)));
            self.bags.insert(name.to_string(), b.clone());
            b
        }
    }

    fn add_bag_from_str(&mut self, bag_str: &str) {
        let input: Vec<&str> = bag_str.split(" bags contain ").collect();
        let bag_name = input[0];

        let bag = self.get_or_create_bag(bag_name);

        if input[1] != "no other bags." {
            for inner_bag in input[1].split(", ") {
                let mut inner_bag_split: Vec<&str> = inner_bag.split(" ").collect();
                let qty: u64 = inner_bag_split.remove(0).parse().unwrap();
                inner_bag_split.pop();
                let mut bag_name = "".to_string();
                for part in inner_bag_split {
                    bag_name += part;
                    bag_name += " ";
                }
                bag_name = bag_name.trim_end().to_string();
                
                let sub_bag = self.get_or_create_bag(&bag_name);
                {
                    let mut mut_bag = sub_bag.borrow_mut();
                    mut_bag.add_containing_bag(qty, bag.clone());
                }
                {
                    let mut mut_bag = bag.borrow_mut();
                    mut_bag.add_held_bag(qty, sub_bag.clone());
                }
            }
        }
        self.bags.insert(bag_name.to_string(), bag);
    }
}

pub fn puzzle1(input: &str) -> i64 {
    let mut chain = BagChain::new();

    for line in input.lines() {
        chain.add_bag_from_str(line);
    }

    let bag = chain.bags.get("shiny gold").unwrap().borrow();
    bag.unique_bag_combos()
}

pub fn puzzle2(input: &str) -> i64 {
    let mut chain = BagChain::new();

    for line in input.lines() {
        chain.add_bag_from_str(line);
    }

    let bag = chain.bags.get("shiny gold").unwrap().borrow();
    bag.total_internal_bags()
}
