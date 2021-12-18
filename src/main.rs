#[macro_use] extern crate text_io;


#[macro_use] extern crate rocket;

use serde::{Deserialize};
use serde::ser::{Serialize, Serializer, SerializeStruct};
use rocket::serde::json::{json, Value};

struct Stacks {
     kiesel_a: i8,
     kiesel_b: i8,
     winner: i8,
     a_sub: i8, 
     b_sub: i8,
     zug : i8, 
}

impl Serialize for Stacks {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Stacks", 3)?;
        state.serialize_field("kiesel_a", &self.kiesel_a)?;
        state.serialize_field("kiesel_b", &self.kiesel_b)?;
        state.serialize_field("winner", &self.winner)?;
        state.serialize_field("a_sub", &self.a_sub)?;
        state.serialize_field("b_sub", &self.b_sub)?;
        state.serialize_field("zug", &self.zug)?;
        state.end()
    }
}


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

    let stapel = Stacks {
     kiesel_a:  13,
     kiesel_b:  17,
     winner: 0,
     a_sub: 0, 
     b_sub: 0,
     zug : 0, 
    };
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

fn get_stapel() -> Stacks{
    return Stacks {
     kiesel_a:  13,
     kiesel_b:  17,
     winner: 0,
     a_sub: 0, 
     b_sub: 0,
     zug : 0, 
    };
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"

}



#[get("/gamestate")]
fn gamestate() -> Value {
    
  json!(get_stapel())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index,gamestate])
}