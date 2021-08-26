fn main() {
    let mut array: [i32; 8] = [0; 8];
    
    array[1] = 1;
    array[2] = 2;
    array[3] = 3;
    array[4] = 4;
    array[5] = 5;
    array[6] = 6;
    array[7] = 7;
    
    assert_eq!([1, 2, 3, 4, 5, 6, 7], &array[1..]);
    
    // This loop prints: 0 1 2 3 4 5 6 7 
    for x in array {
        print!("{} ", x);
    }
    }