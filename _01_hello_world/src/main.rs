fn main() {
    let x : usize = 128;
    println!("{} ist eine Zahl",naiv_gauss(x));
}

fn naiv_gauss(x: usize)->usize{
    if x == 0 {
        0
    }else{
        x+naiv_gauss(x-1)
    }
}