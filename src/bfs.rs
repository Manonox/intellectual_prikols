use std::collections::{VecDeque, BTreeSet, HashMap};
use slab_tree::*;

pub fn normal(start: i32, target: i32, functions: &Vec<fn(i32) -> i32>) -> Vec<i32> {
    let mut closed = BTreeSet::new();
    let mut open  = VecDeque::new();

    let mut tree = TreeBuilder::new().with_root(start).build();
    open.push_back(tree.root_id().unwrap());

    let mut target_node_id: Option<NodeId> = None;
    while let Some(node_id) = open.pop_front() {
        if closed.contains(&node_id) {
            continue
        }

        closed.insert(node_id);
        let mut node_mut = tree.get_mut(node_id).unwrap();
        let value: i32 = *node_mut.data();
        for function in functions {
            let next = function(value);
            let new_node_mut = node_mut.append(next);
            let new_node_id = new_node_mut.node_id();
            open.push_back(new_node_id);
            if next == target {
                target_node_id = Some(new_node_id);
            }
        }
        
        if target_node_id.is_some() {
            break
        }
    }

    let mut path = Vec::new();
    
    while let Some(node_id) = target_node_id {
        let node_ref = tree.get(node_id).unwrap();
        path.push(*node_ref.data());
        target_node_id = node_ref.parent().map(|node| node.node_id());
    }

    path.reverse();
    path
}


pub fn bidirectional(left: i32, right: i32, functions: &Vec<fn(i32) -> i32>, inverse_functions: &Vec<fn(i32) -> i32>) -> Vec<i32> {
    let mut closed_left: HashMap<i32, NodeId> = HashMap::new();
    let mut open_left  = VecDeque::new();

    let mut tree_left = TreeBuilder::new().with_root(left).build();
    open_left.push_back(tree_left.root_id().unwrap());


    let mut closed_right: HashMap<i32, NodeId> = HashMap::new();
    let mut open_right  = VecDeque::new();

    let mut tree_right = TreeBuilder::new().with_root(right).build();
    open_right.push_back(tree_right.root_id().unwrap());


    let mut middle_value_option = None;
    loop {
        if let Some(node_id_left) = open_left.pop_front() {
            let mut node_mut = tree_left.get_mut(node_id_left).unwrap();
            let value: i32 = *node_mut.data();
            
            if !closed_left.contains_key(&value) {
                closed_left.insert(value, node_id_left);

                if closed_right.contains_key(&value) {
                    middle_value_option = Some(value);
                    break;
                }

                for function in functions {
                    let next = function(value);
                    let new_node_mut = node_mut.append(next);
                    let new_node_id = new_node_mut.node_id();
                    open_left.push_back(new_node_id);
                }
            }
        }

        if let Some(node_id_right) = open_right.pop_front() {
            let mut node_mut = tree_right.get_mut(node_id_right).unwrap();
            let value: i32 = *node_mut.data();
            
            if !closed_right.contains_key(&value) {
                closed_right.insert(value, node_id_right);

                if closed_left.contains_key(&value) {
                    middle_value_option = Some(value);
                    break;
                }

                let mut index = 0;
                for function in inverse_functions {
                    let next = function(value);
                    if value != functions[index](next) {
                        continue;
                    }

                    let new_node_mut = node_mut.append(next);
                    let new_node_id = new_node_mut.node_id();
                    open_right.push_back(new_node_id);
                    index += 1;
                }
            }
        }
    }

    let middle_value = middle_value_option.unwrap();

    let mut middle_node_id_left = Some(closed_left[&middle_value]);
    let mut middle_node_id_right = Some(closed_right[&middle_value]);
    
    let mut left_path = Vec::new();
    while let Some(node_id) = middle_node_id_left {
        let node_ref = tree_left.get(node_id).unwrap();
        left_path.push(*node_ref.data());
        middle_node_id_left = node_ref.parent().map(|node| node.node_id());
    }
    
    let mut right_path = Vec::new();
    while let Some(node_id) = middle_node_id_right {
        let node_ref = tree_right.get(node_id).unwrap();
        right_path.push(*node_ref.data());
        middle_node_id_right = node_ref.parent().map(|node| node.node_id());
    }

    left_path.reverse();
    left_path.pop();
    let mut path = Vec::new();
    path.append(&mut left_path);
    path.append(&mut right_path);

    path
}