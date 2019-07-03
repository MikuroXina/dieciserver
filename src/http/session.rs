use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Clone, Copy)]
enum SessionState {
  Established,
  Disconnected,
}

struct Session {
  state: Mutex<SessionState>,
  data: Mutex<String>,
  key: String,
}

impl Session {
  fn new_session(session_key: String) -> Session {
    Session {
      state: Mutex::new(SessionState::Disconnected),
      data: Mutex::new(String::new()),
      key: session_key
    }
  }

  fn access(&self) -> SessionState {
    *self.state.lock().unwrap()
  }

  fn touch(&mut self) {
    *self.state.lock().unwrap()
     = SessionState::Established;
  }

  fn store(&mut self, new_data: String) {
    *self.data.lock().unwrap() = new_data;
  }

  fn get(&self) -> String {
    (*self.data.lock().unwrap()).clone()
  }

  fn clear(&mut self) {
    *self.data.lock().unwrap()
     = String::new();
  }
}

const NUM_SESSIONS: usize = 10000;

struct SessionPool {
  sessions: HashMap<String, Arc<Mutex<Session>>>,
}

impl SessionPool {
  fn new() -> SessionPool {
    SessionPool {
      sessions: HashMap::new()
    }
  }

  fn had(&self, key: &str) -> bool {
    self.sessions.contains_key(key)
  }

  fn get(&mut self, key: &str) -> Arc<Mutex<Session>> {
    match self.sessions.get(key) {
      Some(val) => val.clone(),
      None => {
        {
          let key = key.to_string();
          let new_session = Arc::new(Mutex::new(Session::new_session(key.clone())));
          self.sessions.insert(key, new_session);
        }
        self.sessions.get(key).expect("Failed to add a session").clone()
      }
    }
  }

  fn remove(&mut self, key: &str) -> bool {
    if let Some(_) = self.sessions.remove(key) {
      true
    } else {
      false
    }
  }

  fn compact(&mut self) {
    self.sessions.shrink_to_fit();
  }
}

#[test]
fn senario1() {
  let mut session = Session::new_session("test1".to_string());
  assert_eq!("".to_string(), session.get());
  session.store("test1a".to_string());
  assert_eq!("test1a".to_string(), session.get());
}

#[test]
fn senario2() {
  let mut pool = SessionPool::new();
  assert_eq!(false, pool.had("test2"));
  let mut session = pool.get("test2");
  let mut session = session.lock().unwrap();
  assert_eq!(true, pool.had("test2"));

  // Add noise
  for i in 0..10 {
    pool.get(i.to_string().as_str());
  }

  assert_eq!(true, pool.had("test2"));

  // Add noise part 2
  for i in 0..10 {
    let mut noise_session = pool.get(i.to_string().as_str());
    let mut noise_session = noise_session.lock().unwrap();
    noise_session.store("dummy".to_string());
    pool.remove(i.to_string().as_str());
  }

  assert_eq!(true, pool.had("test2"));
  assert_eq!("".to_string(), session.get());
  session.store("test2a".to_string());
  assert_eq!("test2a".to_string(), session.get());

  // Add noise part 3
  for i in 0..10 {
    let mut noise_session = pool.get(i.to_string().as_str());
    let mut noise_session = noise_session.lock().unwrap();
    noise_session.store("dummy".to_string());
  }

  assert_eq!(true, pool.had("test2"));
}