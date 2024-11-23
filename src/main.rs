fn recursion(perm_number: i32) -> i32 {
    if perm_number != 1 {
        return perm_number * recursion(perm_number - 1);
    } else {
        return 1;
    }
}

fn main() {
    let perm_number: i32 = 3;
    let example = recursion(perm_number);

    println!("the answer of this permutation: {:?}", example);
}
