use crossbeam_channel::{ Receiver, Sender };
use std::io::Read;
use std::thread::JoinHandle;

fn channel_emptying(archive: String, receiver: Receiver<String>, thread: String) -> Result<JoinHandle<()>, std::io::Error> {
 std::thread::Builder::new().name(thread).spawn(
  move || {
   match std::fs::File::open(archive) {
    Ok(file) => {
     match zip::ZipArchive::new(file) {
      Ok(mut items) => {
       loop {
        match receiver.recv() {
         Ok(password) => {
          match items.by_index_decrypt(2, password.as_bytes()) {
           Ok(result) => {
            match result {
             Ok(mut item) => {
              match item.read_to_end(&mut Vec::with_capacity(item.size() as usize)) {
               Ok(_) => {
                println!("Password: {:?}", password);

                break;
               }//Ok(_) => {

               Err(_) => { }
              }//match item.read_to_end(&mut Vec::with_capacity(item.size() as usize)) {
             }//Ok(mut item) => {

             Err(_) => { }
            }//match result {
           }//Ok(result) => {

           Err(error) => { println!("match items.by_index_decrypt(2, password.as_bytes()): {:?}", error); break; }
          }//match items.by_index_decrypt(2, password.as_bytes()) {
         }//Ok(password) => {

         Err(error) => { println!("match receiver.recv(): {:?}", error); break; }
        }//match receiver.recv() {
       }//loop {
      }//Ok(mut items) => {

      Err(error) => { println!("match zip::ZipArchive::new(file): {:?}", error); }
     }//match zip::ZipArchive::new(file) {
    }//Ok(file) => {

    Err(error) => { println!("match std::fs::File::open(archive): {:?}", error); }
   }//match std::fs::File::open(archive) {
  }//move || {
 )//std::thread::Builder::new().name(thread).spawn(
}//fn channel_emptying(archive: String, receiver: Receiver<String>, thread: String) -> Result<JoinHandle<()>, std::io::Error> {

fn channel_filling(passwords: Vec<String>, sender: Sender<String>) -> Result<JoinHandle<()>, std::io::Error> {
 std::thread::Builder::new().name("sender".to_string()).spawn(
  move || {
   for password in passwords {
    match sender.send(password) {
     Ok(_)      => { }
     Err(error) => { println!("match sender.send(password): {:?}", error); break } // channel disconnected, stop thread
    }//match send_sender.send(password) {
   }//for password in passwords {
  }//move || {
 )//std::thread::Builder::new().name("sender".to_string()).spawn(
}//fn channel_filling(passwords: Vec<String>, sender: Sender<String>) -> Result<JoinHandle<()>, std::io::Error> {

fn passwords_creating(chars: String, length: u8) -> Vec<String> {
 let mut passwords: Vec<String> = Vec::new();

 if length > 0 {
  if length > 1 {
   for password in passwords_creating(chars.clone(), length - 1) {
    for char in chars.chars() {
     passwords.push(format!("{}{}", password.clone(), char));

    }//for char in chars.chars() {
   }//for password in passwords {

  } else {//if length > 1 {
   for char in chars.chars() {
    passwords.push(format!("{}", char));

   }//for char in chars.chars() {
  }//} else {//if length > 1 {
 }//if length > 0 {

 passwords
}//fn passwords_creating(chars: String, length: u8) -> Vec<String> {

fn main() {
 if let Some(archive) = std::env::args().nth(1) {
  if let Some(chars) = std::env::args().nth(2) {
   if let Some(length_string) = std::env::args().nth(3) {
    if let Some(threads_string) = std::env::args().nth(4) {
     match length_string.parse::<u8>() {
      Ok(length_u8) => {
       match threads_string.parse::<u8>() {
        Ok(threads_u8) => {
         let (sender, receiver): (Sender<String>, Receiver<String>) = crossbeam_channel::bounded(2000);

         match channel_filling(passwords_creating(chars, length_u8), sender) {
          Ok(thread_sender) => {
           let mut index: u8 = 0;

           let mut threads_recipient: Vec<JoinHandle<()>> = vec![];

           loop {
            index += 1; 

            match channel_emptying(archive.clone(), receiver.clone(), format!("thread_{}", index)) {
             Ok(thread_recipient) => {
              threads_recipient.push(thread_recipient);

             }//Ok(thread_recipient) => {

             Err(error) => { println!("match channel_emptying(archive.clone(), receiver.clone(), format!(thread, index)): {:?}", error); }
            }//match channel_emptying(archive.clone(), receiver.clone(), format!("thread_{}", index)) {

            if index == threads_u8 {
             break;

            }//if index == threads_u8 {
           }//loop {

           for thread_recipient in threads_recipient {
            match thread_recipient.join() {
             Ok(result) => { println!("match thread_recipient.join(), result: {:?}", result); }

             Err(error) => { println!("match thread_recipient.join(): {:?}", error); }
            }//match thread_recipient.join() {
           }//for thread_recipient in threads_recipient {

           match thread_sender.join() {
            Ok(result) => { println!("match thread_sender.join(), result: {:?}", result); }

            Err(error) => { println!("match thread_sender.join(): {:?}", error); }
           }//match thread_sender.join() {
          }//Ok(thread_sender) => {

          Err(error) => { println!("channel_filling(passwords_creating(chars, length_u8), sender): {:?}", error); }
         }//match channel_filling(passwords_creating(chars, length_u8), sender) {
        }//Ok(threads_u8) => {

        Err(error) => { println!("match threads.parse::<u8>(): {:?}", error); }
       }//match threads.parse::<u8>() {
      }//Ok(length_u8) => {

      Err(error) => { println!("match length_string.parse::<u8>(): {:?}", error); }
     }//match length_string.parse::<u8>() {
    }//if let Some(threads_string) = std::env::args().nth(4) {
   }//if let Some(length_string) = std::env::args().nth(3) {
  }//if let Some(chars) = std::env::args().nth(2) {
 }//if let Some(archive) = std::env::args().nth(1) {
}//fn main() {
