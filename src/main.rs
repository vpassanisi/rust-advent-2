fn main() {
    const INPUT: &str = include_str!("./input.txt");

    let lines = INPUT.lines();

    let mut sum = 0;

    for line in lines {
        let v: Vec<&str> = line.split([':', ';', ',']).collect();
        let mut game = 0;
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        let mut pass = true;

        for str in v {
            let d: String = str.chars().filter(|c| c.is_digit(10)).collect();
            let num = d.parse().unwrap();
            if str.contains("Game") {
                game = num;
            } else if str.contains("red") && num > red {
                red = num;
            } else if str.contains("green") && num > green {
                green = num;
            } else if str.contains("blue") && num > blue {
                blue = num;
            }
        }
        if red > 12 {
            pass = false;
        }
        if green > 13 {
            pass = false;
        }
        if blue > 14 {
            pass = false;
        }

        if pass {
            sum += game;
        }
    }

    println!("{}", sum)
}
