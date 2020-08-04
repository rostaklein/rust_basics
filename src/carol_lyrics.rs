pub fn carol_lyrics() {
    let mut song: String = String::new();

    const DAYS: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    const GIFTS: [&str; 12] = [
        "a partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings, badam-pam-pam",
        "Six geese a laying",
        "Seven swans a swimming",
        "Eight maids a milking",
        "Nine ladies dancing",
        "Ten lords a leaping",
        "Eleven pipers piping",
        "12 drummers drumming",
    ];

    for (day_num, day) in DAYS.iter().enumerate() {
        song.push_str("On the ");
        song.push_str(day);
        song.push_str(" day of Christmas\n");
        song.push_str("My true love gave to me\n");

        for gift_num in (0..day_num + 1).rev() {
            if day_num != 0 && gift_num == 0 {
                song.push_str("And ");
            }
            song.push_str(GIFTS[gift_num]);
            song.push_str("\n");
        }
        song.push_str("\n");
    }

    println!("{}", song)
}
