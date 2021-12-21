#[macro_use]
extern crate rocket;

use rocket::serde::json::{json, Value};
use rocket::State;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::sync::Mutex;
//currently no Arc
//not in use yet:
use rocket::request::{self, FromRequest, Request};
use serde::Deserialize;

struct Stacks {
    lock: Mutex<()>,
    kiesel_a: i8,
    kiesel_b: i8,
    winner: i8,
    a_sub: i8,
    b_sub: i8,
    zug: i32,
    message: String,
}

impl Stacks {
    pub fn new() -> Stacks {
        Stacks {
            lock: Mutex::new(()),
            kiesel_a: 13,
            kiesel_b: 17,
            winner: 0,
            a_sub: 0,
            b_sub: 0,
            zug: 0,
            message: format!(
                "\nKieselspiel: Wer den letzten Stein nimmt, verliert.
\nKieselspiel: Wer den letzten Stein nimmt, verliert.\n
Die Züge werden im Format a b eingegeben,\n
Es dürfen immer nur von einem Stapel beliebig viele,\n
oder von beiden Stapeln gleichviele Steine entfernt werden."
            ),
        }
    }
    fn ziehen(&mut self) {
        let _lock = self.lock.lock().unwrap();
        //Held until end of block
        self.zug += 1;
    }

    fn sub_a(&mut self, sub: i8) {
        let _lock = self.lock.lock().unwrap();
        //Held until end of block
        self.kiesel_a -= sub;
    }

    fn sub_b(&mut self, sub: i8) {
        let _lock = self.lock.lock().unwrap();
        //Held until end of block
        self.kiesel_b -= sub;
    }

    fn set_message(&mut self, message: String) {
        let _lock = self.lock.lock().unwrap();
        //Held until end of block
        self.message = message;
    }
}

impl Serialize for Stacks {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 6 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Stacks", 7)?;
        state.serialize_field("kiesel_a", &self.kiesel_a)?;
        state.serialize_field("kiesel_b", &self.kiesel_b)?;
        state.serialize_field("winner", &self.winner)?;
        state.serialize_field("a_sub", &self.a_sub)?;
        state.serialize_field("b_sub", &self.b_sub)?;
        state.serialize_field("zug", &self.zug)?;
        state.serialize_field("message", &self.message)?;
        state.end()
    }
}

fn winsign(player: i8) {
    println!("Spieler {0} hat gewonnen!!!", player);
}

fn get_stapel() -> Stacks {
    return Stacks::new();
}

#[get("/gamestate?show&<takeaway>")]
fn gamestate(takeaway: Option<i8>) -> Value {
    let mut a: Stacks;
    match takeaway {
        Some(b) => {
            a = get_stapel();
            a.kiesel_a -= b;
        }
        None => a = get_stapel(),
    }
    json!(a)
}

#[launch]
fn rocket() -> _ {
    let status = Mutex::new(Stacks::new());
    rocket::build()
        .mount("/", routes![index, gamestate, count, modularstate])
        .manage(status)
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
#[get("/count")]
fn count(state: &State<Mutex<Stacks>>) -> String {
    let mut _lock = state.inner().lock().unwrap();
    _lock.ziehen();
    format!("Number of visits: {}", _lock.zug)
}

#[get("/modularstate?move&<rem_a>&<rem_b>")]
fn modularstate(state: &State<Mutex<Stacks>>, rem_a: Option<i8>, rem_b: Option<i8>) -> Value {
    let mut _lock = state.inner().lock().unwrap();
    match rem_a {
        Some(a) => match rem_b {
            Some(b) => {
                if (a <= _lock.kiesel_a && b <= _lock.kiesel_b) && (b == a || b == 0 || a == 0) {
                    _lock.ziehen();
                    _lock.sub_a(a);
                    _lock.sub_b(b);
                } else {
                    let movenr = _lock.zug;
                    _lock.set_message(format!(
                        "Spieler {0} hat eine ungültige Eingabe getätigt,",
                        (movenr % 2 + 1)
                    ));
                }
            }

            None => println!("es wurden keine Steine entfernt (B-Falsch)."),
        },
        None => println!("es wurden keine Steine entfernt (A-Falsch)."),
    }
    //Wincheck
    if 0 == (_lock.kiesel_a + _lock.kiesel_b) {
        if _lock.zug % 2 == 0 {
            _lock.winner = 1;
        } else {
            _lock.winner = 2;
        }
        winsign(_lock.winner);
    }
    let printable = Stacks {
        lock: Mutex::new(()),
        kiesel_a: _lock.kiesel_a,
        kiesel_b: _lock.kiesel_b,
        winner: _lock.winner,
        a_sub: _lock.a_sub,
        b_sub: _lock.b_sub,
        zug: _lock.zug,
        message: format!("{0}", _lock.message),
    };
    json!(printable)
}
