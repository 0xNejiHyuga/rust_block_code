// block code
fn main() {
    {
        let x = 10;
        println!("{x}");
        {
            let y = 15;
            println!("{y}")
        }
    }
    println!("After block code");

}
