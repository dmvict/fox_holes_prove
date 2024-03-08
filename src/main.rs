fn main() {
    let mut fox: i16 = i16::MAX;
    let length: i16 = 5;
    let upper_bound_mask: i16 = i16::MAX >> (16 - (length + 1));

    let steps = [2, 2, 3, 4, 4, 3, 2];
    for i in steps.iter() {
        let rsh = fox >> 1;
        let lsh = fox << 1;
        let res = rsh | lsh;
        let masked = res & upper_bound_mask;

        println!("\nStep::{}", i);
        let s = format!("{:05b}", masked).chars().rev().collect::<String>();
        println!("{}", s);
        fox = masked & !(1 << (i - 1));
        let mut s = format!("{:05b}", fox).chars().rev().collect::<String>();
        s.replace_range(*i - 1..*i, "Y");
        println!("{}\n", s);

        if fox == 0 {
            println!("You win");
        }
    }
}
