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
            //Appends each value in list to the veclist
            veclist.push(number);
        }
    }

    // Prints the typed list
    println!("{:?}", veclist);

    // Finds list length
    let len = veclist.len();

    // This function returns a vector with the sorted list;
    quicksort(&mut veclist, 0, len - 1);

    // Prints the sorted list
    println!("{:?}", veclist);
}

fn quicksort(list: &mut [i32], start: usize, end: usize) {
    // Checks if sorted yet
    if end <= start {
        return;
    }

    // Only for debugging purposes
    let workingsegment: &mut[i32] = &mut list[start..=end];
    println!("start: {:?}, end: {:?}, list: {:?}", start, end, workingsegment);

    // Find median-of-three and make it our pivot
    let first = list[start];
    let last = list[end];
    let middle = list[(start + end) / 2];
    println!("first: {:?}, middle: {:?}, last: {:?}", first, middle, last);
    let pivot = middle.clamp(first.min(last), first.max(last));

    // Sets pivot to the index of the previously found median
    let mut pivot: usize = 
        if pivot == first {start} 
        else if pivot == middle {(start + end) / 2} 
        else {end};

    println!("pivot: {:?}", pivot);
    // Swap pivot and last
    swap(list, pivot, end);
    println!("list after first swap: {:?}", list);
    pivot = end;
    let mut left: usize = start;
    let mut right: usize = end - 1;

    loop {
        // Finds index of left
        if let Some(index) = (left..end).find(|&i| list[i] >= list[pivot]) {
            left = index;
        }   else {
                left = end - 1;
            }

        // Finds index of right
        if let Some(index) = (start..=right).rev().find(|&i| list[i] < list[pivot]) {
            right = index;
        }   else {
                right = end - 1;
            }

        if left < right {
            swap(list, left, right);
            println!("Swap {:?} and {:?}", left, right);
            println!("{:?}", list);
            // Force progress
            left += 1;
            right -= 1;
        } else {
            if list[left] > list[pivot] {
                println!("Before swapping the pivot with left: {:?}", list);  
                swap(list, left, pivot);
                pivot = left;
                println!("After swapping the pivot with left: {:?}", list);

                // For debug
                println!("left: {:?}, right: {:?}, end: {:?}", left, right, end);

                // Ensures pivot is not on the far left so it doesn't try and take what is to the left of it (nothing)
                if pivot >= 1 {
                    quicksort(list, start, pivot - 1);
                }
                quicksort(list, pivot + 1, end);
            }
            return;
        }
    }
}

fn swap(list: &mut [i32], idx1: usize, idx2: usize) {
    let value1 = list[idx1];
    list[idx1] = list[idx2];
    list[idx2] = value1;
}