#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
//extern crate diesel; 

use rocket::request::Form;
use rocket::http::{RawStr,Status};
use rocket::request::Request;
use rocket::response::content::Html;
use rocket::response::content::Css;
use rocket::response::{Responder, NamedFile,Redirect};
use rocket::response::status::Custom;
use rocket::{Outcome,Data};
use rocket::Outcome::Failure;
use rocket::http::Cookies;
use rocket::http::Cookie;

use std::path::{Path,PathBuf};
use std::collections::HashMap;
use std::net::SocketAddr;

use auth::*;

pub mod auth;

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
fn teapot() -> Status{
    Status::ImATeapot
}


#[get("/style.css?<version>")]
fn stylev2(version : u32) -> NamedFile{
    NamedFile::open(Path::new("./resources/style.css")).unwrap()
}

#[get("/")]
fn start() -> NamedFile{
    NamedFile::open(Path::new("./resources/presenze.html")).unwrap()
}

#[derive(FromForm,Debug)]
struct Email{
    email : String
}

#[derive(FromForm,Debug)]
struct PostForm{
    email : String,
    vcode : String
}
impl PostForm{
    fn set_email(&mut self, email : String) {
        self.email = email;
    }
}

#[post("/confirm", data="<email>")]
fn confirm(email : Form<Email>, mut cookies : Cookies, addr : SocketAddr) -> NamedFile{
    println!("{:?}", addr);
    cookies.add(Cookie::new("email",email.email.clone()));
    NamedFile::open(Path::new("./resources/confirm.html")).unwrap()
}

#[post("/submit", data="<data>" )]
fn submit(data : Form<PostForm>,  cookies : Cookies) ->Redirect {
    let mut dat = data.into_inner();
    dat.set_email(cookies.get("email").unwrap().value().to_owned());
    println!("{:?}",dat);
    Redirect::to(uri!(register_result))
}

#[get("/notfound")]
fn ntf() -> Status{
    Status::NotFound
}


#[get("/<path..>", rank = 1)]
fn general(path : PathBuf) -> Result<NamedFile, Status>{
    let named = NamedFile::open(Path::new("./resources/").join(path));
    match named{
        Ok(nam) => Ok(nam),
        Err(_) => Err(Status::Gone)
    }
}

#[get("/result")]
fn register_result(mut cookies : Cookies) -> NamedFile{
    let cook = cookies.get("email");
    match cook {
        None => {
            println!("Failed, cookie for email not found for this connection");
            NamedFile::open(Path::new("./resources/failure.html")).unwrap()
        },
        Some(cooki) =>{
            println!("Success for email: {}", cooki.value());
            cookies.remove(Cookie::named("email"));
            NamedFile::open(Path::new("./resources/success.html")).unwrap()
        }
    }
} 


#[catch(418)]
fn teap(req : &Request) -> Html<&'static str>{
    Html("<!DOCTYPE html>
            <html>
            <head>
                <meta charset=\"utf-8\">
                <title>I'm A Teapot</title>
            </head>
            <body align=\"center\">
                <div align=\"center\">s
					 <img src=\"teapot.jpg\" alt=Teapot>
                </div>
            </body>
            </html>")
}

#[catch(500)]
fn saat(re : &Request) -> String{
    format!("Blocked attempt to summoning nasal daemons at {}", re.uri())
} 


#[get("/satan")]
fn satan() -> Custom<&'static str>{
    Custom(Status::new(666, "Hail Satan!"), "Porcoddio!")
}

fn main() {
  //  let conn = MysqlConnection::establish(db)
    //    .expect(&format!("Error connecting to {}", db));
    rocket::ignite().register(catchers![teap])
        .mount("/", routes![teapot, stylev2,start,general,confirm,submit,satan, register_result, ntf]).launch();
}
