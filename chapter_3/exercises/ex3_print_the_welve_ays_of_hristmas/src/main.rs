fn main() {
    print_music_lyrcis();
}

fn print_music_lyrcis() {
    const NUMER_OF_LINES: usize = 12;
    const TEXT: [&str; NUMER_OF_LINES] = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three french hens",
        "Two turtle doves and",
        "A partridge in a pear tree",
    ];

    for phrase_number in 1..NUMER_OF_LINES + 1 {
        println!(
            "On the {}. day of Christmas, my true love sent to me",
            phrase_number
        );

        for line_numer in NUMER_OF_LINES - phrase_number..NUMER_OF_LINES {
            println!("{}", TEXT[line_numer]);
        }
        println!("\n");
    }
}
