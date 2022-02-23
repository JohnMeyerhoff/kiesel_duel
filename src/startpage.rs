use rocket::response::content::Html;
pub fn get_homepage() -> Html<&'static str> {
    Html(
        r"
        <html>
        <head>
        <title>Kiesel Duell</title>
        <style>
        div.green{
    background-color:green;
}
td div.content {
    position:relative;
    display: inline-block;
}
td div.fill{
    display: inline-block;
    height:100%;
    position:absolute;
    top:0px;
}
</style>
        </head>
        
        </br>
        <body>Hello, you may visit <a href=http://127.0.0.1:8000/modularstate?move&rem_a=10&rem_b=10>this link</a> to play the game!
        <div class='fill green' style='margin: 0 auto; width:800px; height:600px'>

        </div>
        </body>
        </html>",
    )
}

#[allow(dead_code)]
pub fn get_simple_homepage() -> Html<&'static str> {
    Html(
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
