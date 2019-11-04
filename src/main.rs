#![feature(proc_macro_hygiene, decl_macro, vec_remove_item)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
//extern crate diesel; 

use rocket::http::{RawStr,Status};
use rocket::request::{Request, Form, State};
use rocket::response::content::{Html, Css};
use rocket::response::{Responder, NamedFile,Redirect};
use rocket::response::status::Custom;
use rocket::{Outcome,Data};
use rocket::Outcome::Failure;
use rocket::http::{Cookies , Cookie};

use std::path::{Path,PathBuf};
use std::collections::HashMap;
use std::net::SocketAddr;

use auth::*;
use auth::{Token, Session, SessionsHolder};
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
fn confirm(email : Form<Email>, mut cookies : Cookies, addr : SocketAddr, holder: State<SessionsHolder>) -> NamedFile{
    println!("{:?}", addr);
    println!("{:?}", email);
    let guest = holder.auth_guest_session(email.email.clone());
    //TODO is possible to register the token for the custom guard without using directly the cookies
    cookies.add(Cookie::new("auth_token", guest.get_token().get_value()));
    NamedFile::open(Path::new("./resources/confirm.html")).unwrap()
}


fn register_result(session : Session, form : PostForm, holder : State<SessionsHolder>) -> NamedFile{
    
//  NamedFile::open(Path::new("./resources/failure.html")).unwrap()
    holder.deauth_session(session);
    NamedFile::open(Path::new("./resources/success.html")).unwrap()
    
    //TODO proper registration and controls
      
}
 


#[post("/submit", data="<data>" )]
fn submit(data : Form<PostForm>,  sess_token : Token, holder : State<SessionsHolder>, mut cookies : Cookies) ->Result<NamedFile,Status> {
    let mut dat = data.into_inner();
    println!("{:?}", sess_token);
    let res =  match holder.get_by_token(sess_token){
        None => Err(Status::Forbidden),
        Some(sessi) =>{
            println!("{:?}",dat);
            Ok(register_result(sessi, dat, holder))
        }
    };
    cookies.remove(Cookie::named("auth_token"));
    res
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
    let mut sessions_holder = SessionsHolder::new(false);
    rocket::ignite().manage(sessions_holder).register(catchers![teap])
        .mount("/", routes![teapot, stylev2,start,general,confirm,submit,satan, ntf]).launch();
}
