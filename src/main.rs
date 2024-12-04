mod domain;
mod infra;
mod usecase;
mod util;

fn main() {
    let user = infra::RegisterMailMagazineCommand {
        first_name: String::from("Wojciech"),
        last_name: String::from("Jedynak"),
        email: String::from("wojciech_jedynak@lexxpluss.com"),
    };
}
