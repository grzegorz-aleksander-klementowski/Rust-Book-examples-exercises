// I wrote the example from the book on my own

fn largest_num(array: &[u32]) -> &u32 {
    let mut largest = &array[0];

    for item in array {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(array: &[char]) -> &char {
    let mut largest = &array[0];

    for item in array {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    // In the Rust book it's much shorter
    let my_number_list = vec![
        734, 12, 98, 451, 23, 876, 54, 302, 19, 640, 88, 901, 267, 45, 711, 3, 592, 134, 420, 66,
        999, 17, 281, 73, 560, 408, 91, 250, 804, 38, 612, 7, 489, 156, 930, 64, 321, 845, 29, 570,
        402, 18, 760, 95, 214, 683, 41, 890, 127, 355, 502, 9, 774, 68, 290, 640, 81, 913, 47, 166,
    ];

    let res_largest_num = largest_num(&my_number_list);
    println!("The largest number is: {res_largest_num}");

    let my_number_list = vec![
        821, 45, 176, 392, 67, 945, 112, 287, 31, 708, 154, 832, 349, 59, 690, 14, 573, 198, 461,
        83, 917, 26, 305, 141, 624, 377, 108, 294, 859, 52, 681, 21, 458, 173, 902, 97, 336, 814,
        63, 599, 417, 34, 742, 129, 268, 655, 74, 921, 185, 344, 519, 16, 788, 104, 273, 661, 92,
        884, 58, 201,
    ];

    let res_largest_num = largest_num(&my_number_list);
    println!("The largest number is: {res_largest_num}");
}
