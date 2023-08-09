fn main()
{
    let a = 0;
    let b = &mut a;
    *b += 1;
    println!("{}", *b);
}
