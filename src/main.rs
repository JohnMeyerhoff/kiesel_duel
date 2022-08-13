#[macro_use]
extern crate rocket;
use rocket::fs::NamedFile;
use rocket::response::content::RawHtml;
use rocket::serde::json::{json, Value};
use rocket::Shutdown;
use rocket::State;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::sync::Mutex;

struct Stacks {
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
            kiesel_a: 13,
            kiesel_b: 17,
            winner: 0,
            a_sub: 0,
            b_sub: 0,
            zug: 0,
            message: format!(
                "\nKieselspiel: Wer den letzten Stein nimmt, verliert.\n
Die Züge werden im Format a b eingegeben,\n
Es dürfen immer nur von einem Stapel beliebig viele,\n
oder von beiden Stapeln gleichviele Steine entfernt werden."
            ),
        }
    }

    fn ziehen(&mut self) {
        //Held until end of block
        self.zug += 1;
    }

    fn sub_a(&mut self, sub: i8) {
        self.kiesel_a -= sub;
    }

    fn sub_b(&mut self, sub: i8) {
        self.kiesel_b -= sub;
    }

    fn set_message(&mut self, message: String) {
        self.message = message;
    }
    fn set_winner(&mut self, winner: i8) {
        self.winner = winner;
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

#[launch]
fn rocket() -> _ {
    let status = Mutex::new(Stacks::new());
    rocket::build()
        .mount("/", routes![index, count, modularstate, newgame, shutdown])
        .manage(status)
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("src/frontend.html").await.ok()
}

#[get("/count")]
fn count(state: &State<Mutex<Stacks>>) -> String {
    let mut _lock = state.inner().lock().unwrap();
    _lock.ziehen();
    format!("Number of visits: {}", _lock.zug)
}

#[get("/shutdown")]
fn shutdown(shutdown: Shutdown) -> &'static str {
    shutdown.notify();
    "Shutting down..."
}

#[get("/newgame")]
fn newgame(state: &State<Mutex<Stacks>>) -> RawHtml<&'static str> {
    let mut _lock = state.inner().lock().unwrap();
    *_lock = Stacks::new();
    println!("Spielstand zurueckgesetzt!");
    return RawHtml(
        r"
<html>
<head>
<title>Kiesel Duell</title>
</head>
<body>Hello, a new game session started!
</body>
</html>",
    );
}

#[get("/modularstate?move&<rem_a>&<rem_b>")]
fn modularstate(state: &State<Mutex<Stacks>>, rem_a: Option<i8>, rem_b: Option<i8>) -> Value {
    let mut _lock = state.inner().lock().unwrap();
    if _lock.winner == 0 {
        match rem_a {
            Some(a) => match rem_b {
                Some(b) => {
                    if (a <= _lock.kiesel_a && b <= _lock.kiesel_b)
                        && (b == a || b == 0 || a == 0)
                        && ((b + a) != 0)
                    {
                        _lock.ziehen();
                        _lock.sub_a(a);
                        _lock.sub_b(b);
                        let player = (_lock.zug % 2 + 1) as i8;
                        let ak = _lock.kiesel_a;
                        let bk = _lock.kiesel_b;
                        if ak == 0 && bk == 0 {
                            _lock.set_winner(player);
                            winsign(player);
                            _lock.set_message(format!("Spieler {0} hat gewonnen!!!", player));
                        } else {
                            _lock.set_message(format!(
                                "Spieler {0} ist am zug mit A: {1} und B: {2} ",
                                player, ak, bk
                            ));
                        }
                    } else {
                        let movenr = _lock.zug;
                        let player = (movenr % 2 + 1) as i8;
                        let ak = _lock.kiesel_a;
                        let bk = _lock.kiesel_b;
                        _lock.set_message(format!(
                            "Spieler {3} hat eine ungültige Eingabe getätigt , es wurden keine Steine entfernt.\nSpieler {0} ist am zug mit A: {1} und B: {2} ",
                            player, ak, bk,(movenr % 2 + 1)
                        ));
                    }
                }
                None => println!("es wurden keine Steine entfernt (B-Falsch)."),
            },
            None => println!("es wurden keine Steine entfernt (A-Falsch)."),
        }
        let printable = Stacks {
            kiesel_a: _lock.kiesel_a,
            kiesel_b: _lock.kiesel_b,
            winner: _lock.winner,
            a_sub: _lock.a_sub,
            b_sub: _lock.b_sub,
            zug: _lock.zug,
            message: format!("{0}", _lock.message),
        };
        json!(printable)
    } else {
        let printable = Stacks {
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
}
