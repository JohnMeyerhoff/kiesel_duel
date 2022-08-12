use rocket::response::content::RawHtml;
pub fn get_homepage() -> RawHtml<&'static str> {
    RawHtml(
        r"<!DOCTYPE html>
<html lang='en'>

<head>
    <meta charset='UTF-8'>
    <meta http-equiv='X-UA-Compatible' content='IE=edge'>
    <meta name='viewport' content='width=device-width, initial-scale=1.0'>
    <title>Kiesel Duel</title>
    <style>
        div.green {
            background-color: green;
        }




        bluebox {
            float: left;
            height: 20px;
            width: 20px;
            margin-bottom: 15px;
            border: 1px solid black;
            clear: both;
            background-color: blue;
        }

        redbox {
            float: left;
            height: 20px;
            width: 20px;
            margin-bottom: 15px;
            border: 1px solid black;
            clear: both;
            background-color: red;
        }

        td div.content {
            position: relative;
            display: inline-block;
        }

        td div.fill {
            display: inline-block;
            height: 100%;
            position: absolute;
            top: 0px;
        }
    </style>
    <script>
        function loadDoc() {
            var xhttp = new XMLHttpRequest();
            xhttp.onreadystatechange = function () {
                if (this.readyState == 4 && this.status == 200) {
                    updateView(this.responseText)
                }
            };
            a = document.getElementById('arem').value;
            b = document.getElementById('brem').value;
            xhttp.open('GET', 'http://127.0.0.1:8000/modularstate?move&rem_a=' + a + '&rem_b=' + b, true);
            xhttp.send();
        }
        function restartView(){
           document.getElementById('demo').innerHTML ='Spieler 1 ist am zug mit A: 13 und B: 17 ';
        }
        function newgame() {
            var xhttp = new XMLHttpRequest();
            xhttp.onreadystatechange = function () {
                if (this.readyState == 4 && this.status == 200) {
                    restartView();
                }
            };
            xhttp.open('GET', 'http://127.0.0.1:8000/newgame', true);
            xhttp.send();
        }
        function updateView(raw) {
            x = JSON.parse(raw);
            document.getElementById('demo').innerHTML = x.message;
            // {'a_sub':0,'b_sub':0,'kiesel_a':2,'kiesel_b':11,'message':'Spieler 2 ist am zug mit A: 2 und B: 11 ','winner':0,'zug':7}
        }
    </script>
</head>

<body>
    <h1>Kiesel Duel</h1>

    </head>

    </br>

    <body>

        <div class='fill green' style='margin: 0 auto; width:800px; height:600px'>
            <h3>Kiesel A:</h3>
            <h3 id='demo'>Content</h3>
            <input type='number' name='name' id='arem' value='0' />
            <input type='number' name='name' id='brem' value='0' />
            <button type='button' onclick='loadDoc()'>move</button>

            <button type='button' style='float:right' onclick='newgame()'>restart</button>
            </br></br>
            <div id='playfield'>
                <!-- vertically split the playfield in two -->
                <div id='left' style='width:50%;float:left'>
                    <bluebox></bluebox>
                </div>

                <div id='right' style='width:50%;float:right'>
                    <redbox></redbox>
                </div>
            </div>
        </div>
    </body>

</html>",
    )
}

#[allow(dead_code)]
pub fn get_simple_homepage() -> RawHtml<&'static str> {
    RawHtml(
        r"
        <html>
        <head>
        <title>Kiesel Duell</title>
        </head>
        <body>Hello, you may visit <a href=http://127.0.0.1:8000/modularstate?move&rem_a=10&rem_b=10>this link</a> to play the game!
        </body>
        </html>",
    )
}
