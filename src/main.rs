#[macro_use] extern crate text_io;


#[macro_use] extern crate rocket;
use rocket::response::content;
use serde::{Serialize, Deserialize};

fn ma0in() {
    println!("\nKieselspiel: Wer den letzten Stein nimmt, verliert.");
    println!("Die Züge werden im Format a b eingegeben,");
    println!("Es dürfen immer nur von einem Stapel beliebig viele,");
    println!("oder von beiden Stapeln gleichviele Steine entfernt werden.");
    let mut kiesel_a: i8 = 13;
    let mut kiesel_b: i8 = 17;
    let mut winner: i8;
    let mut a_sub: i8; 
    let mut b_sub: i8;
    let mut zug = 0; 
    while 0<(kiesel_a+kiesel_b) {
        if zug % 2 == 0 {
            winner = 1;
        }else {
            winner =2;
        }
        println!("Spieler {0} am zug mit A={1} und B={2}:",winner,kiesel_a,kiesel_b);
        a_sub = read!();
        b_sub = read!();
        if (a_sub<=kiesel_a && b_sub <=kiesel_b) && (b_sub == a_sub || b_sub == 0 || a_sub == 0) {
            kiesel_b = kiesel_b - b_sub;
            kiesel_a = kiesel_a - a_sub;
            zug = zug + 1;
        } else{
         println!("Spieler {0} hat eine ungültige Eingabe getätigt,",winner);
         println!("es wurden keine Steine entfernt.");
        }
    }
        if zug % 2 == 0 {
            winner = 1;
        }else {
            winner =2;
        }
    winsign(winner);
}
fn winsign(player:i8){
    println!("Spieler {0} hat gewonnen!!!",player);
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"

}

#[get("/gamestate")]
fn gamestate() -> content::Json<&'static str> {
  content::Json("{
    'status': 'success',
    'A': '17',
    'B': '13',
    'turn': '1',
    'message': 'There is no message',
  }")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/state", routes![gamestate])
}