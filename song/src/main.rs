fn main() {
   fn convert_to_fahrenheit(celsius: f64) -> f64 {
       celsius * 1.8 + 32.0
   } 

   fn fibonaci(n: u32) -> u32 {
       if n == 0 {
           return 0;
       } else if n == 1 {
           return 1;
       } else {
           return fibonaci(n - 1) + fibonaci(n - 2);
       }
   }
   fn print_lyrics_for_the_twelve_days_of_christmas() {
    let days: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    for i in 0..12 {
        println!("On the {} day of Christmas \nmy true love sent to me", days[i]);
        for j in (0..i + 1).rev() {
            if j == 0 && i != 0 {
                print!("And ");
            }
            println!("{}", gifts[j]);
        }
        println!();
    }           
   }
   print_lyrics_for_the_twelve_days_of_christmas();
}   
