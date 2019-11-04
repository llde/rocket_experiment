use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use std::sync::atomic::{AtomicU32,AtomicBool, Ordering};
use std::sync::{Arc,Mutex};
use std::thread::Thread;


use rocket::http::{Status};
use rocket::request::{FromRequest,Request, Outcome};


#[derive(Eq,PartialEq,Clone)]
pub enum Role{
    User,
    Admin,
    Guest
}

#[derive(Eq,PartialEq,Clone,Debug)]
pub enum TokenError{
    Invalid,     
    NotExist
}

#[derive(Clone,PartialEq,Eq, Debug)]
pub struct Token{
    t : String
}

impl Token{
    pub fn get_value(&self) -> String{ self.t.clone()}
}


impl<'a, 'r> FromRequest<'a, 'r> for Token {
    type Error = TokenError;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let cookies = request.cookies();
        let tok_res = cookies.get("auth_token");
        match tok_res {
            Some(token) => Outcome::Success(Token{t : token.value().to_string()}),
            None => Outcome::Failure((Status::Forbidden, TokenError::NotExist))
        } 
        
    }
}

#[derive(Clone,PartialEq,Eq)]
pub struct Session{
    username : String,
    role : Role,
    token : Token,
    time_to_live : u32
}



impl Session{
    
    pub fn create_session(user : String, role : Role) -> Session{
        if role != Role::Guest {unimplemented!()}
        let mut hasher = DefaultHasher::new();
        user.hash(&mut hasher);
        let token = Token{t : hasher.finish().to_string() };
        println!("{:?}", token);
        //TODO render token actually more robust using JWT
        Session{username : user, role : role, token : token , time_to_live : 120 *  60 }
    }
    
    pub fn delete_session(self) -> () { }  //Drop ?
    
    pub fn renew_session(&mut self, token : String){unimplemented!()}
    
    pub fn get_token(&self) -> Token{self.token.clone()}
    
    pub fn get_user(&self) -> String {self.username.clone()}

}


pub struct SessionsHolder{
    sessions_counter : AtomicU32,
    sessions : Arc<Mutex<Vec<Session>>>,
    collector : Option<Thread>,
    collect_garbage_sessions : AtomicBool

}

//TODO transactional holder?
impl SessionsHolder{
    pub fn new(collect_garbage_sessions : bool) -> SessionsHolder{
        if collect_garbage_sessions == true {unimplemented!()}
        let mut sessions = Arc::new(Mutex::new(Vec::new()));
        SessionsHolder{sessions_counter : AtomicU32::new(0), sessions: sessions, collector : Option::None, collect_garbage_sessions : AtomicBool::new(false)}
    
    }
    
    pub fn autenthicate_user(user : String, hpassw : String){unimplemented!()}
    
    pub fn auth_guest_session(&self, user : String) -> Session{
        let session = Session::create_session(user, Role::Guest);
        (*self.sessions.lock().unwrap()).push(session.clone());
        self.sessions_counter.fetch_add(1, Ordering::SeqCst);
        session
    }
    
    pub fn deauth_session(&self, session : Session) -> Option<Session>{
        self.sessions_counter.fetch_sub(1, Ordering::SeqCst);
        (*self.sessions.lock().unwrap()).remove_item(&session)
        //TODO result and vocal error if not present
    }
    
    pub fn get_by_token(&self, token : Token) -> Option<Session>{  
        let guard = self.sessions.lock().unwrap();
        let mut session = None;
        for ses in guard.iter(){
            if ses.token == token{
                session = Some(ses.clone());
                break;
            }
        }
        session
    }
    
    pub fn drop_all_sessions(&mut self){unimplemented!()}
    
}



