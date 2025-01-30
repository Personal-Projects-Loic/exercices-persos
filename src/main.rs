mod exercice1;
mod exercice2;
use exercice1::{swap_any_type, swap_i32, SomethingToDisplay};
use exercice2::Rectangle;

fn main() {
    // exo 1
    let mut type1 = String::from("no more fortnite");
    let mut type2 = String::from("yes more fortnite");
    let un_truc = SomethingToDisplay {
        test: "eggy".to_string(),
        comment: "worst than fornite".to_string(),
    };

    un_truc.print();

    swap_i32(42, 24);
    swap_any_type(&mut type1, &mut type2);

    // exo 2
    let rec = Rectangle::new(42, 24).unwrap();
    println!("Rectangle's area : {}", rec.area());
}
