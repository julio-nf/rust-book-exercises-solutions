fn main() {
    let days = [
        ("first", 1),
        ("second", 2),
        ("third", 3),
        ("fourth", 4),
        ("fifth", 5),
        ("sixth", 6),
        ("seventh", 7),
        ("eighth", 8),
        ("ninth", 9),
        ("tenth", 10),
        ("eleventh", 11),
        ("twelfth", 12),
    ];

    let gifts = [
        "A partridge in a pear tree.",
        "Two turtle doves, and",
        "Three French hens,",
        "Four colly birds,",
        "Five gold rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lord a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    for day in days {
        println!("On the {} day of Christmas,", day.0);
        println!("My true love sent to me");

        for i in (0..day.1).rev() {
            println!("{}", gifts[i]);
        }

        println!();
    }
}
