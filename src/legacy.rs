
macro_rules! page_template {
    ($name:expr, $description:expr) => (
        concat!(r#"
            <!DOCTYPE html>
            <html>
            <head>
                <meta charset="utf-8">
                <title>"#, $name, r#"</title>
            </head>
            <body align="center">
                <div align="center">
                    <h1>"#,$name, r#"</h1>
                    <p>"#, $description, r#"</p>
                    <hr />
                    <small>Rocket</small>
                </div>
            </body>
            </html>
        "#
        )
    )
}

#[get("/killergame")]
fn killer() -> NamedFile{
    NamedFile::open(Path::new("./resources/killergame.html")).unwrap()
}
#[get("/killergame/about")]
fn kg_about() -> NamedFile{
    NamedFile::open(Path::new("./resources/about.html")).unwrap()
}

#[get("/login")]
fn kg_login() -> NamedFile{
    NamedFile::open(Path::new("./resources/login.html")).unwrap()
}


#[get("/gg")]
fn gg() -> HTML<&'static str>{
    HTML(page_template!("GG", "Git Gud"))
} 


#[derive(FromForm)]
struct Query{
    renew : bool
}
#[get("/style.css?<query>")]
fn style_q(query : Query) -> NamedFile{
    println!("{:?}", std::env::current_dir().unwrap());
    NamedFile::open(Path::new("./resources/style.css")).unwrap()
}

#[error(599)]
fn satan(req : &Request) -> HTML<&'static str>{
    HTML(error_page_template!(666, "Blasphemy", "Satan was here"))
}


#[get("/error")]
fn error() -> Failure{
    Failure(Status{code : 599, reason : "Satan was here"})
}
