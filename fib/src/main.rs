use std::io;
fn main() {
    let n = 2;
    let mut F_nm2:u32 = 1;
    let mut F_nm1:u32 = 1;
    loop {
        let F = F_nm2 + F_nm1;
        println!("F_{n} = {F}");
        F_nm2 = F_nm1;
        F_nm1 = F;
        let mut _null = String::new();
        io::stdin().read_line(&mut _null);
    }
}
