// Creation of a min heap
// a max heap is the same thing but opposite

fn get_left_child_index(parent_index: usize) -> usize {
    2 * parent_index + 1
}
fn get_right_child_index(parent_index: usize) -> usize {
    2 * parent_index + 2
}
fn get_parent_index(child_index: usize) -> Option<usize> {
    match child_index.checked_sub(2) {
        Some(x) => x.checked_div(2),
        None => None,
    }
}

fn swap(index1: usize, index2: usize, items: &mut Vec<usize>) {
    let temp = items[index1];
    items[index1] = items[index2];
    items[index2] = temp;
}

fn main() {
    // let vals = [3, 4, 8, 9, 7, 10, 9, 15, 20, 13];

    let mut items: Vec<usize> = Vec::with_capacity(10);
    let mut capacity = 10;
    let mut size = 0;

    println!("parent_index: {:?}", get_parent_index(3))
}
