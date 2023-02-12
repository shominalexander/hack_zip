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
       let mut index: usize = 0;

       'passwords: loop {
        match receiver.recv() {
         Ok(password) => {
          loop {
           match items.by_index_decrypt(index, password.as_bytes()) {
            Ok(result) => {
             match result {
              Ok(mut item) => {
               match item.read_to_end(&mut Vec::with_capacity(item.size() as usize)) {
                Ok(_) => {
                 println!("Password: {:?}", password);

                 break 'passwords;
                }//Ok(_) => {

                Err(_) => { break; }
               }//match item.read_to_end(&mut Vec::with_capacity(item.size() as usize)) {
              }//Ok(mut item) => {

              Err(_) => { break; }
             }//match result {
            }//Ok(result) => {

            Err(error) => { println!("match items.by_index_decrypt({:?}, password.as_bytes()): {:?}", index, error); index += 1; if index > 5 { break 'passwords; } }
           }//match items.by_index_decrypt(index, password.as_bytes()) {
          }//loop {
         }//Ok(password) => {

         Err(error) => { println!("match receiver.recv(): {:?}", error); break; }
        }//match receiver.recv() {
       }//'outer: loop {
      }//Ok(mut items) => {

      Err(error) => { println!("match zip::ZipArchive::new(file): {:?}", error); }
     }//match zip::ZipArchive::new(file) {
    }//Ok(file) => {

    Err(error) => { println!("match std::fs::File::open(archive): {:?}", error); }
   }//match std::fs::File::open(archive) {
  }//move || {
 )//std::thread::Builder::new().name(thread).spawn(
}//fn channel_emptying(archive: String, receiver: Receiver<String>, thread: String) -> Result<JoinHandle<()>, std::io::Error> {

fn channel_filling(chars: String, length: u8, sender: Sender<String>) -> Result<JoinHandle<()>, std::io::Error> {
 let mut all: Vec<String> = Vec::new();

 let mut index: u8 = 0;

 let mut part: Vec<String> = Vec::new();

 if length > 0 {
  if length > 1 {
   while index < length {
    if part.len() > 0 {
     for item in &part {
      for char in chars.chars() {
       all.push(format!("{}{}", item, char));

      }//for char in chars.chars() {
     }//for item in &part {

    } else {//if part.len() > 0 {
     for char in chars.chars() {
      all.push(format!("{}", char));

     }//for char in chars.chars() {
    }//} else {//if part.len() > 0 {

    part = all.clone();

    index += 1;
   }//while index < length {

  } else {//if length > 1 {
   for char in chars.chars() {
    all.push(format!("{}", char));

   }//for char in chars.chars() {
  }//} else {//if length > 1 {
 }//if length > 0 {

 std::thread::Builder::new().name("sender".to_string()).spawn(
  move || {
   for password in all {
    match sender.send(password) {
     Ok(_)      => { }
     Err(error) => { println!("match sender.send(password): {:?}", error); break } // channel disconnected, stop thread
    }//match send_sender.send(password) {
   }//for password in all {
  }//move || {
 )//std::thread::Builder::new().name("sender".to_string()).spawn(
}//fn channel_filling(chars: String, length: u8, sender: Sender<String>) -> Result<JoinHandle<()>, std::io::Error> {

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

         match channel_filling(chars, length_u8, sender) {
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
             Ok(_) => { }

             Err(error) => { println!("{:?}", error); }
            }//match thread_recipient.join() {
           }//for thread_recipient in threads_recipient {

           match thread_sender.join() {
            Ok(_) => { }

            Err(error) => { println!("{:?}", error); }
           }//match thread_sender.join() {
          }//Ok(thread_sender) => {

          Err(error) => { println!("channel_filling(passwords_creating(chars, length_u8), sender): {:?}", error); }
         }//match channel_filling(chars, length_u8, sender) {
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
