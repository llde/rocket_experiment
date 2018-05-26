#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_codegen;
extern crate rocket_contrib;
//extern crate diesel; 

//use diesel::prelude::*;
//use diesel::mysql::MysqlConnection;
use rocket::response::Failure;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::content::Html;
use rocket::response::content::Css;
use rocket::response::{Responder, Result, NamedFile};
use rocket_contrib::Template;
use std::path::{Path,PathBuf};
use std::collections::HashMap;


static db : &'static str = "mysql://root@localhost:3306/hello-rocket";
macro_rules! error_page_template {
    ($code:expr, $name:expr, $description:expr) => (
        concat!(r#"
            <!DOCTYPE html>
            <html>
            <head>
                <meta charset="utf-8">
                <title>"#, $code, " ", $name, r#"</title>
            </head>
            <body align="center">
                <div align="center">
                    <h1>"#, $code, ": ", $name, r#"</h1>
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



#[get("/teapot")]
fn teapot() -> Failure{
    Failure(Status::ImATeapot)
}

#[get("/pollina")]
fn pollina() -> Failure{
    Failure(Status{code :444, reason : ""})
}

#[get("/user/<name>")]
fn index(name : String) -> Html<String> {
    let name = format!("{} page", name);
    let body = format!("{}", name);
    let desc = format!("This is {} personal page", name);
    Html(format!( "
            <!DOCTYPE html>
            <html>
            <head>
                <meta charset=\"utf-8\">
                <title>{}</title>
            </head>
            <body align=\"center\">
                <div align=\"center\">
                    <h1> {} </h1>
                    <p> {} </p>
                    <hr />
                    <small>Rocket</small>
                </div>
            </body>
            </html>
        ", name,body,desc))
    
}

#[derive(FromForm)]
struct Version{
    v : u32
}
#[get("/style.css?<version>")]
fn stylev2(version : Version) -> NamedFile{
    NamedFile::open(Path::new("./resources/style.css")).unwrap()
}

#[get("/")]
fn start() -> NamedFile{
    NamedFile::open(Path::new("./resources/pastebin.html")).unwrap()
}

#[get("/gen_err/<code>")]
fn gen_err(code : u32) -> Template{
    let sttt = format!("{}",code);
    let mut context : HashMap<&str,&str>= HashMap::new();
    context.insert("error_code", &sttt);
    context.insert("reason", "TBD");
    Template::render("error", &context)
}


#[get("/<path..>", rank = 1)]
fn general(path : PathBuf) -> std::io::Result<NamedFile>{
    NamedFile::open(Path::new("./resources/").join(path))
}

#[get("/satan")]
fn satan() -> Failure{
    Failure(Status{code : 599, reason : "Satan was here"})
}

#[get("/god")]
fn god() -> Html<&'static str>{
    Html("<!DOCTYPE html>
            <html>
            <head>
                <meta charset=\"utf-8\">
                <title>Streaming video</title>
            </head>
            <body align=\"center\">
                <div align=\"center\">
					 <video src=Mon.webm>
                </div>
            </body>
            </html>")
}

#[error(418)]
fn teap(req : &Request) -> Html<&'static str>{
    Html("<!DOCTYPE html>
            <html>
            <head>
                <meta charset=\"utf-8\">
                <title>I'm A Teapot</title>
            </head>
            <body align=\"center\">
                <div align=\"center\">
					 <img src=\"teapot.jpg\" alt=Teapot>
                </div>
            </body>
            </html>")
}

#[error(599)]
fn satanism(req : &Request) -> Html<&'static str>{
    Html(error_page_template!(666, "Blasphemy", "Satan was here"))
}


#[error(444)]
fn pol(req : &Request) -> Html<&'static str>{
    Html(error_page_template!(444, "Pollina", "Iddio Pollina!"))
}

fn main() {
  //  let conn = MysqlConnection::establish(db)
    //    .expect(&format!("Error connecting to {}", db));
    rocket::ignite().catch(errors![pol,teap,satanism])
        .mount("/", routes![index,teapot, stylev2,pollina,gen_err,start, general,satan,god]).attach(Template::fairing())
        .launch();
}