// I wrote the example from the book on my own

fn main() {
    // In the Rust book it's much shorter
    let my_number_list = vec![
        734, 12, 98, 451, 23, 876, 54, 302, 19, 640, 88, 901, 267, 45, 711, 3, 592, 134, 420, 66,
        999, 17, 281, 73, 560, 408, 91, 250, 804, 38, 612, 7, 489, 156, 930, 64, 321, 845, 29, 570,
        402, 18, 760, 95, 214, 683, 41, 890, 127, 355, 502, 9, 774, 68, 290, 640, 81, 913, 47, 166,
    ];

    let mut largest = &my_number_list[0];

    for num in &my_number_list {
        if num > largest {
            largest = num
        }
    }
}
