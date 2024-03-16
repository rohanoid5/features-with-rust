/*
 We’ll be given n lists that are all sorted in ascending order of popularity rank.
 We have to combine these lists into a single list that will be sorted by rank in ascending order,
 meaning from best to worst.
*/

use std::collections::LinkedList;

pub fn merge_two_lists(list1: &LinkedList<i32>, list2: &LinkedList<i32>) -> LinkedList<i32> {
    let mut merged_list: LinkedList<i32> = LinkedList::new();

    let mut iter1 = list1.iter();
    let mut iter2 = list2.iter();

    let mut item1 = iter1.next();
    let mut item2 = iter2.next();

    while item1 != None && item2 != None {
        if item1 < item2 {
            merged_list.push_back(*item1.unwrap());
            item1 = iter1.next();
        } else {
            merged_list.push_back(*item2.unwrap());
            item2 = iter2.next();
        }
    }

    if item1 != None {
        while item1 != None {
            merged_list.push_back(*item1.unwrap());
            item1 = iter1.next();
        }
    } else {
        while item2 != None {
            merged_list.push_back(*item2.unwrap());
            item2 = iter2.next();
        }
    }

    merged_list
}

pub fn driver() {
    let mut a: LinkedList<i32> = LinkedList::new();

    a.push_back(11);
    a.push_back(41);
    a.push_back(51);

    let mut b: LinkedList<i32> = LinkedList::new();
    b.push_back(21);
    b.push_back(23);
    b.push_back(42);

    let mut c: LinkedList<i32> = LinkedList::new();
    c.push_back(25);
    c.push_back(56);
    c.push_back(66);
    c.push_back(72);

    let mut lists: Vec<LinkedList<i32>> = Vec::new();
    lists.push(c);
    lists.push(a);
    lists.push(b);

    lists.sort_by(|a, b| a.front().cmp(&b.front()));

    let mut merged_list: LinkedList<i32> = lists[0].to_owned();

    for index in 1..lists.len() {
        merged_list = merge_two_lists(&merged_list, &lists[index]);
    }

    println!("Merged list: {merged_list:?}");
}
