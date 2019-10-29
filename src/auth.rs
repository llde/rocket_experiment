
use std::sync::atomic::{AtomicU32,AtomicBool};
use std::sync::{Arc,Mutex};
use std::thread::Thread;


pub enum Role{
    User,
    Admin,
    Guest
}



struct Token{
    t : String
}

struct Session{
    id: u32,
    username : String,
    role : Role,
    token : Token,
    time_to_live : u32
}



impl Session{
    
    pub fn create_session(user : String, role : Role){unimplemented!()}
    
    pub fn delete_session(self){unimplemented!()}  //Drop ?
    
    pub fn renew_session(&mut self, token : String){unimplemented!()}

}

struct SessionsHolder{
    sessions_counter : AtomicU32,
    sessions : Arc<Mutex<Vec<Session>>>,
    collector : Option<Thread>,
    collect_garbage_sessions : AtomicBool

}


impl SessionsHolder{
    pub fn new(collect_garbage_sessions : bool) -> SessionsHolder{unimplemented!()}
    
    pub fn autenthicate_user(user : String, hpassw : String){unimplemented!()}
    
    pub fn deauth_session(){unimplemented!()}
    
    pub fn get_by_token(&mut self, token : String) -> Option<Session>{unimplemented!()}
    
    pub fn drop_all_sessions(&mut self){unimplemented!()}
    
}



