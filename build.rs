extern crate cc;

fn main() {
    cc::Build::new().file("src/winverhelper.c").compile(
        "winverhelper",
    );
}
