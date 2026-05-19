fn main() {
    // The list that we will use to find median and mode
    let list = "4 9 3 8 2";
    println!("{}", list);

    // This function returns a vector with the sorted list;
    let sortedlist = sortlist(list);
}

fn sortlist(list: &str) -> Vec<i32> {
    let mut listsorter = Vec::new();
    for word in list.split_whitespace() {
        let number = word.parse().expect("uh oh");
    }
    for n in list {
        listsorter.push(n)
    }
    println!("{:?}", listsorter);
    listsorter
}