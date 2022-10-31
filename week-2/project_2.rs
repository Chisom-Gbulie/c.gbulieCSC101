fn main() {
//t represents Toshiba, m represents Mac, h represents Hp, d represents Dell, a represents Acer

    let m1 = 2;
    let m2 = 1;
    let m3 = 3;
    let m4 = 3;
    let m5 = 1;

    let v1 = 450_000;
    let v2 = 1_500_000;
    let v3 = 750_000;
    let v4 = 2_850_000;
    let v5 = 250_000;

    let t = m1 * v1;
    let m = m2 * v2;
    let h = m3 * v3;
    let d = m4 * v4;
    let a = m5 * v5;

//s is the sum of sales record, b is the average of sales record
    let s = t + m+ h + d + a;
    println!("Sum of sales record is {}",s);
    let b = s / 5;
    println!("Average of sales record is {}",b);
}
