use std::fs::read_to_string;


fn trebuchet_part1(filename: &str) 
{
    let mut sum = 0;

    for line in read_to_string(filename).unwrap().lines() 
    {
        let mut digit:String = String::new();

        for c in line.to_string().chars()
        {
            if c.is_numeric()
            {
                digit.push(c);
                break;
            }
        }

        for c in line.to_string().chars().rev()
        {
            if c.is_numeric()
            {
                digit.push(c);
                sum += digit.parse::<u32>().unwrap();
                break;
            }
        }
    }

    println!("Sum of part 1: {}", sum);
}


fn trebuchet_part2(filename: &str)
{
    let mut sum = 0;

    for line in read_to_string(filename).unwrap().lines()
    {
        let mut digit:String = String::new();

        let modified_line = line
            .replace("one", "o1e")
            .replace("two", "t2")
            .replace("three", "t3e")
            .replace("four", "f4")
            .replace("five", "f5e")
            .replace("six", "s6")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e");

        for c in modified_line.to_string().chars()
        {
            if c.is_numeric()
            {
                digit.push(c);
                break;
            }
        }

        for c in modified_line.to_string().chars().rev()
        {
            if c.is_numeric()
            {
                digit.push(c);
                break;
            }
        }

        sum += digit.parse::<u32>().unwrap();
    }

    println!("Sum of part 2: {}", sum);
}


fn main()
{
    trebuchet_part1("./files/input.txt");
    trebuchet_part2("./files/input.txt");
}