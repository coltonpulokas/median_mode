use std::io;

fn main() {
    // The list that we will use to find median and mode
    let mut list = String::new();

    // New vector to sort the list
    let mut veclist: Vec<i32> = Vec::new();

    // Asks user to type a list of integers
    println!("Enter a list of integers.");

    // Keeps trying to take user input and put into vector until we get a non empty vector
    while let [] = veclist.as_slice() {
        io::stdin() 
            .read_line(&mut list)
            .expect("Failed to read line");
        println!("{}", list);

        // Take each slice between whitespace and put it in vector
        for word in &mut list.split_whitespace() {
            let number = match word.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter integers seperated by whitespace.");
                    // Overwrites veclist to an empty vector
                    veclist.clear();
                    // Overwrites list to an empty string
                    list.clear();
                    break;
                }
            };
            veclist.push(number);
        }
    }

    // Prints the typed list
    println!("{:?}", veclist);

    // Finds list length
    let len = veclist.len();

    // This function returns a vector with the sorted list;
    let sortedlist: Vec<i32> = quicksort(&mut veclist, 0, len - 1);

    // Prints the sorted list
    println!("{:?}", sortedlist);
}

fn quicksort(list: &mut [i32], start: usize, end: usize) -> Vec<i32> {
    // Checks if sorted yet
    if end <= start {
        return list.to_vec();
    }

    let list: &mut[i32] = &mut list[start..(end + 1 + start)];

    println!("start: {:?}, end: {:?}, list: {:?}", start, end, list);

    // Find median-of-three and make it our pivot
    let [first, .., last] = list[..] else {
        return list.to_vec();
    };
    let middle = list[end / 2];
    println!("first: {:?}, middle: {:?}, last: {:?}", first, middle, last);
    let pivot = middle.clamp(first.min(last), first.max(last));

    // Sets pivot to the index of the previously found median
    let mut pivot: usize = 
        if pivot == first {0} 
        else if pivot == middle {end / 2} 
        else {end};

    println!("pivot: {:?}", pivot);
    // Swap pivot and last
    swap(list, pivot, end);
    pivot = end;
    let mut left: usize = 0;
    let mut right: usize = end - 1;
    loop {
        // Finds index of left
        while list[left] < list[pivot] {
            if left < (end - 1) {
                left += 1;
            } else {
                break;
            }
        }
        //let left = match list.iter().position(|&x| x >= list[pivot]) {
        //    Some(ind) => ind,
        //   None => pivot,
        //;

        // Finds index of right
        while list[right] > list[pivot] {
            if right > 0 {
                right -= 1;
            } else {
                break;
            }
        }
        //let right = match list.iter().rposition(|&x| x <= list[pivot]) {
        //    Some(ind) => ind,
        //    None => 0,
        //};

        if left < right {
            swap(list, left, right);
            println!("Swap {:?} and {:?}", left, right);
            println!("{:?}", list);
            // Force progress
            left += 1;
            right -= 1;
        } else {
            println!("Before swapping the pivot back: {:?}", list);  
            swap(list, left, pivot);
            println!("After swapping the pivot back: {:?}", list);

            // For debug
            println!("left: {:?}, right: {:?}, end: {:?}", left, right, end);

            let sortedlist = [quicksort(list, 0, left - 1), (&[list[left]]).to_vec(), quicksort(list, left + 1, end - (left + 1))].concat();
            return sortedlist
        }
    }
}

fn swap(list: &mut [i32], idx1: usize, idx2: usize) {
    let value1 = list[idx1];
    list[idx1] = list[idx2];
    list[idx2] = value1;
}