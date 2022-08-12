use rocket::response::content::RawHtml;
pub fn get_homepage() -> RawHtml<&'static str> {
    RawHtml(
        r"
        <!DOCTYPE html>
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
                    document.getElementById('demo').innerHTML = this.responseText;
                }
            };
            a = document.getElementById('arem').value;
            b = document.getElementById('brem').value;
            xhttp.open('GET', 'http://127.0.0.1:8000/modularstate?move&rem_a='+a+'&rem_b=' + b, true);
            xhttp.send();
        }
    </script>
</head>

<body>
    <h1>Kiesel Duel</h1>

    </head>

    </br>

    <body>Hello, you may visit <a href=modularstate?move&rem_a=10&rem_b=10>this link</a> to play
        the game!
        <div class='fill green' style='margin: 0 auto; width:800px; height:600px'>
            <span>Kiesel A:</span>
            <div id='demo'>Content</div>
            <input type='number' name='name' id='arem' value='0' />
            <input type='number' name='name' id='brem' value='0' />
            <button type='button' onclick='loadDoc()'>Change Content</button>
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
