extern crate winver;

fn main() {
    println!("Is XP or greater? {}", winver::is_winxp());
    println!("Is XP SP1 or greater? {}", winver::is_winxp_sp1());
    println!("Is XP SP2 or greater? {}", winver::is_winxp_sp2());
    println!("Is XP SP3 or greater? {}", winver::is_winxp_sp3());
    println!("Is Vista or greater? {}", winver::is_winvista());
    println!("Is Vista SP1 or greater? {}", winver::is_winvista_sp1());
    println!("Is Vista SP2 or greater? {}", winver::is_winvista_sp2());
    println!("Is 7 or greater? {}", winver::is_win7());
    println!("Is 7 SP1 or greater? {}", winver::is_win7_sp1());
    println!("Is 8 greater? {}", winver::is_win8());
    println!("Is 8.1 or greater? {}", winver::is_win8_p1());
    println!("Is 10 or greater? {}", winver::is_win10());
    println!(
        "Is Windows Threshold or greater? {}",
        winver::is_winthreshold()
    );
    println!("Is Server Edition? {}", winver::is_winserver());
}
