#[macro_use] extern crate text_io;
#[macro_use] extern crate rocket;

use serde::{Deserialize};
use serde::ser::{Serialize, Serializer, SerializeStruct};
use rocket::serde::json::{json, Value};
use rocket::State;
use rocket::request::{self, Request, FromRequest};


struct Stacks {
     kiesel_a: i8,
     kiesel_b: i8,
     winner: i8,
     a_sub: i8, 
     b_sub: i8,
     zug : i8, 
}

impl Stacks{
    pub fn new() -> Self {
        let kiesel_a: i8 = 13;
        let kiesel_b: i8 = 17;
        let winner: i8= 0;
        let a_sub: i8=0; 
        let b_sub: i8=0;
        let zug = 0;

        Stacks {
            kiesel_a,  
            kiesel_b,
            winner,
            a_sub, 
            b_sub,
            zug,
        }
    }
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
    let mut status = Stacks::new();
    while 0<(status.kiesel_a+status.kiesel_b) {
        if status.zug % 2 == 0 {
            status.winner = 1;
        }else {
            status.winner =2;
        }
        println!("Spieler {0} am zug mit A={1} und B={2}:",status.winner,status.kiesel_a,status.kiesel_b);
        status.a_sub = read!();
        status.b_sub = read!();
        if (status.a_sub<=status.kiesel_a && status.b_sub <=status.kiesel_b) && (status.b_sub == status.a_sub || status.b_sub == 0 || status.a_sub == 0) {
            status.kiesel_b = status.kiesel_b - status.b_sub;
            status.kiesel_a = status.kiesel_a - status.a_sub;
            status.zug = status.zug + 1;
        } else{
         println!("Spieler {0} hat eine ungültige Eingabe getätigt,",(status.zug % 2 +1));
         println!("es wurden keine Steine entfernt.");
        }
    }
        if status.zug % 2 == 0 {
            status.winner = 1;
        }else {
            status.winner =2;
        }
    winsign(status.winner);
}

fn winsign(player:i8){
    println!("Spieler {0} hat gewonnen!!!",player);
}

fn get_stapel() -> Stacks{
    return Stacks::new();
}


#[get("/gamestate?show&<takeaway>")]
fn gamestate(takeaway:Option<i8>) -> Value {
    let mut a : Stacks;
    match takeaway {
        Some(b) => {
            a = get_stapel();
            a.kiesel_a-=b;
        },
        None => a = get_stapel(),
    }
    json!(a)
}

#[launch]
fn rocket() -> _ {
    let status = Stacks::new();
    rocket::build()
    .mount("/", routes![index,gamestate])
    .manage(status)
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"

}

