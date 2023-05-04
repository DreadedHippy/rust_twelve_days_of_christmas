//Declare the twelve days in an array
const TWELVE_DAYS: [&str; 12] = [
    "A partridge in a pear tree",
    "Two turtle doves",
    "Three french hen",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",  
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming"
];

fn main() {
    let x: usize = TWELVE_DAYS.len();

    for i in 1..=x {
        let position : &str = match i {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th"
        };
        
        println!("");
        println!("On the {}{} day of christmas, my true love sent to me", i, position);

        let mut j = i;

        while j > 0 {
            let mut and = "";

            if i != 1 && j == 1 {
                and = "and ";
            };

            println!("{}{}",and, TWELVE_DAYS[j-1]);

            j -= 1;
        }
        println!("");
        println!("------------------------------------");
    }
}
