// use near_contract_standards::fungible_token::events;
use near_sdk::{
    borsh::*,
    env,
    serde::{Deserialize, Serialize},
    *,
  };
  // Declearing account
  pub type AccountId = String;
  
  use std::collections::HashMap;
  
  //a  struct to help handle errors in the contract
  #[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize)]
  #[serde(crate = "near_sdk::serde")]
  pub struct ContractErr(String);
  #[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize)]
  #[serde(crate = "near_sdk::serde")]
  pub struct SuccefullMsg(String);
  
  #[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize)]
  #[serde(crate = "near_sdk::serde")]
  pub struct Event {
    event_id: usize,
    eventname: String,
    started_time: String,
    ended_time: String,
    users: Vec<User>,
  }
  impl Event {
    fn new_event(
      event_id: usize,
      eventname: String,
      started_time: String,
      ended_time: String,
    ) -> Self {
      Self {
        event_id,
        eventname,
        started_time,
        ended_time,
        users: Vec::new(),
      }
    }
    fn add_user_to_event(&mut self, user: User) {
      self.users.push(user)
    }
    fn view_users_in_event(&self) -> &Vec<User> {
      &self.users
    }
  }
  // Struct for creating users who have to join evnets
  #[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Serialize, Deserialize)]
  #[serde(crate = "near_sdk::serde")]
  pub struct User {
    name: String,
    username: String,
    email: String,
    user_id: String,
  }
  impl User {
    fn new_user(name: String, username: String, email: String) -> Self {
      Self {
        name,
        username,
        email,
        user_id: env::signer_account_id().to_string(),
      }
    }
  }
  // struct to declear events and users
  
  #[near_bindgen]
  #[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize)]
  #[serde(crate = "near_sdk::serde")]
  pub struct SmartEvent {
    events: HashMap<String, Event>,
    users: Vec<User>,
  }
  
  impl Default for SmartEvent {
    fn default() -> Self {
      Self {
        events: HashMap::new(),
        users: Vec::new(),
      }
    }
  }

  // Smart contract begin here
  #[near_bindgen]
  impl SmartEvent {
    pub fn new_event() -> Self {
      let events: HashMap<String, Event> = HashMap::new();
      let users: Vec<User> = Vec::new();
  
      SmartEvent { events, users }
    }
  
    // Public method to count events
    pub fn count_events(&self) -> usize {
      self.events.len()
    }
  
    // Public method to count users
    pub fn count_users(&self) -> usize {
      self.users.len()
    }
  
    // Public method to create events and save them in a vector
    pub fn create_event(
      &mut self,
      eventname: String,
      started_time: String,
      ended_time: String,
    ) -> SuccefullMsg {
      let len = self.events.len();
      let event_id = len + 1;
      let event = &eventname.clone();
      let event1 = Event::new_event(event_id, eventname, started_time, ended_time);
      //insering event to the map of events
      self.events.insert(event1.eventname.clone(), event1);
  
      let msg = format!("Your ID is{}", &event_id);
      log!(&msg);
      SuccefullMsg(format!("{}: Event was created succesfully", &event))
    }
  
    // Methods to display events
    pub fn show_events(&mut self) -> Vec<&Event> {
      let events = self.events.iter().map(|e| e.1).collect();
      events
    }
  
    // Public method to create users and save them in vectors
    pub fn check_in_user(&mut self, name: String, username: String, email: String) -> SuccefullMsg {
      let user1 = User::new_user(name, username, email);
      self.users.push(user1);
      SuccefullMsg("User Created Succesfully".to_string())
    }
    // Method to display users
    pub fn get_users(&self) -> &Vec<User> {
      &self.users
    }
  
    // Method which allow user to checkin into event
    pub fn add_new_user_to_event(
      &mut self,
      eventname: &String,
      name: String,
      username: String,
      email: String,
    ) -> Result<SuccefullMsg, ContractErr> {
      match self.events.get_mut(eventname) {
        Some(event) => {
          let user1 = User::new_user(name, username, email);
          event.add_user_to_event(user1);
          let msg = format!("User succefully added to {} event", &eventname);
          Ok(SuccefullMsg(msg))
        }
        None => Err(ContractErr("Failed to get event".to_string())),
      }
    }
  
    // USer checkin for a specific event
    pub fn check_in_exiting_user_to_event(
      &mut self,
      username: String,
      eventname: String,
    ) -> Result<SuccefullMsg, ContractErr> {
      //loopin through the existing users
      for user_ind in 0..self.users.len() {
        //getting the user index
        let user = &mut self.users[user_ind];
        //matching if the username at the index is equal to entered user
        if user.username == username {
          match self.events.get_mut(&eventname) {
            Some(event) => {
              event.add_user_to_event(user.clone());
              let msg = format!("User succefully checked in  to {} event", &eventname);
              return Ok(SuccefullMsg(msg));
            }
            None => return Err(ContractErr("event doesnt exit".to_string())),
          }
        } else {
          return Err(ContractErr("no such user".to_string()));
        }
      }
      Err(ContractErr("Failed".to_string()))
    }
  
    // Method shows users who checked in into the specific event
    pub fn view_users_in_event(&self, eventname: &String) -> Result<&Vec<User>, ContractErr> {
      match self.events.get(eventname) {
        Some(e) => Ok(&e.view_users_in_event()),
        None => Err(ContractErr(
          "Failed to get users for that event".to_string(),
        )),
      }
    }
  }
  
  // use the attribute below for unit tests
  #[cfg(test)]
  mod tests {
    use super::*;
    use near_sdk::{test_utils::*, AccountId};
  
    fn get_context(account: AccountId) -> VMContextBuilder {
      let mut bulder = VMContextBuilder::new();
      bulder.signer_account_id(account);
      return bulder;
    }
  
    // Method to test evnets
    #[test]
    fn add_event() {
      let username = AccountId::new_unchecked("djrefuge.testnet".to_string());
      let _context = get_context(username.clone());
  
      let mut contract = SmartEvent::new_event();
      let msg = contract.create_event(
        "BlockChain event".to_string(),
        "10:30 pm".to_string(),
        "12:30 pm".to_string(),
      );
      println!("{:?}", msg);
      assert_eq!(contract.count_events(), 1);
    }
  
    // Method to test users
    #[test]
    fn create_user() {
      let username = AccountId::new_unchecked("djrefuge.testnet".to_string());
      let _context = get_context(username.clone());
      let mut contract = SmartEvent::new_event();
      contract.check_in_user(
        "refuge".to_string(),
        "homie".to_string(),
        "refuge@gmail.com".to_string(),
      );
      let results = contract.count_users();
      assert_eq!(results, 1);
    }
  
    #[test]
    fn get_users() {
      let username = AccountId::new_unchecked("djrefuge.testnet".to_string());
      let _context = get_context(username.clone());
      let mut contract = SmartEvent::new_event();
      contract.check_in_user(
        "refuge".to_string(),
        "homie".to_string(),
        "refuge@gmail.com".to_string(),
      );
      contract.check_in_user(
        "musa".to_string(),
        "wagole".to_string(),
        "wagole@gmail.com".to_string(),
      );
  
      let count = contract.get_users();
      assert_eq!(count.len(), 2);
    }
  }