use std::io::Read;

struct Password { chars           : String
                , indices_in_chars: Vec<usize>
                }

impl Password {
 fn incriment(&mut self) -> bool {
  let     chars_amount     : usize = self.chars.len()           ;
  let mut index_in_password: usize = 0                          ;
  let     password_size    : usize = self.indices_in_chars.len();

  while index_in_password < password_size {
   if index_in_password < password_size - 1 {
    if self.indices_in_chars[index_in_password] < chars_amount - 1 {
     self.indices_in_chars[index_in_password] += 1;

     return true;

    } else {//if self.indices_in_chars[index_in_password] < chars_amount - 1 {
     self.indices_in_chars[index_in_password] = 0;

     index_in_password += 1;
    }//} else {//if self.indices_in_chars[index_in_password] < chars_amount - 1 {

   } else {//if index_in_password < password_size - 1 {
    if self.indices_in_chars[index_in_password] < chars_amount - 1 {
     self.indices_in_chars[index_in_password] += 1;

     return true;

    } else {//if self.indices_in_chars[index_in_password] < chars_amount - 1 {
     index_in_password += 1;
    
    }//} else {//if self.indices_in_chars[index_in_password] < chars_amount - 1 {
   }//} else {//if index_in_password < password_size - 1 {
  }//while index_in_password < password_size {

  index_in_password = 0;

  while index_in_password < password_size {
   self.indices_in_chars[index_in_password] = 0;

   index_in_password += 1;
  }//while index_in_password < password_size {

  return false;
 }//fn incriment(&mut self) -> bool {

 fn make(&mut self) -> String {
  let mut password: String = String::new();

  for index_in_chars in &self.indices_in_chars {
   if let Some(char) = self.chars.chars().nth(*index_in_chars) {
    password = format!("{}{}", password, char);

   }//if let Some(char) = self.chars.chars().nth(*index_in_chars) {
  }//for index_in_chars in &self.indices_in_chars {

  password
 }//fn make(&mut self) -> String {

 fn new(chars: String, password_size: usize) -> Self {
  let mut index_in_password: usize = 0;

  let mut password = Password { chars: chars.clone(), indices_in_chars: Vec::new() };

  while index_in_password < password_size {
   password.indices_in_chars.push(0usize);
   
   index_in_password += 1;
  }//while index_in_password < password_size {

  password
 }//fn new(chars: String, password_size: usize) -> Self {
}//impl Password {

fn channel_emptying(archive: String, notification_sender: std::sync::mpsc::Sender<String>, thread: String, verification_receiver: crossbeam_channel::Receiver<String>) -> Result<std::thread::JoinHandle<()>, std::io::Error> {
 std::thread::Builder::new().name(thread).spawn(
  move || {
   match std::fs::File::open(archive) {
    Ok(file) => {
     match zip::ZipArchive::new(file) {
      Ok(mut items) => {
       let mut index: usize = 0;

       'passwords: loop {
        match verification_receiver.recv() {
         Ok(password) => {
          loop {
           match items.by_index_decrypt(index, password.as_bytes()) {
            Ok(result) => {
             match result {
              Ok(mut item) => {
               match item.read_to_end(&mut Vec::with_capacity(item.size() as usize)) {
                Ok(_) => {
                 match notification_sender.send(password) {
                  Ok(_) => {
                   break 'passwords;

                  }//Ok(result) => {

                  Err(error) => { println!("match send_password_for_notification.send(password): {:?}", error); }
                 }//match notification_sender.send(password) {

                 break 'passwords;
                }//Ok(_) => {

                Err(_) => { break; }
               }//match item.read_to_end(&mut Vec::with_capacity(item.size() as usize)) {
              }//Ok(mut item) => {

              Err(_) => { break; }
             }//match result {
            }//Ok(result) => {

            Err(error) => { println!("items.by_index_decrypt({:?}, password): {:?}", index, error); index += 1; if index > 5 { break 'passwords; } }
           }//match items.by_index_decrypt(index, password.as_bytes()) {
          }//loop {
         }//Ok(password) => {

         Err(error) => { println!("match receiver.recv(): {:?}", error); break; }
        }//match verification_receiver.recv() {
       }//'outer: loop {
      }//Ok(mut items) => {

      Err(error) => { println!("match zip::ZipArchive::new(file): {:?}", error); }
     }//match zip::ZipArchive::new(file) {
    }//Ok(file) => {

    Err(error) => { println!("match std::fs::File::open(archive): {:?}", error); }
   }//match std::fs::File::open(archive) {
  }//move || {
 )//std::thread::Builder::new().name(thread).spawn(
}//fn channel_emptying(archive: String, notification_sender: std::sync::mpsc::Sender<String>, thread: String, verification_receiver: crossbeam_channel::Receiver<String>) -> Result<std::thread::JoinHandle<()>, std::io::Error> {

fn channel_filling(chars: String, password_size: usize, verification_sender: crossbeam_channel::Sender<String>) -> Result<std::thread::JoinHandle<()>, std::io::Error> {
 std::thread::Builder::new().name("sender".to_string()).spawn(
  move || {
   let mut password: Password = Password::new(chars.clone(), password_size);

   loop {
    match verification_sender.send(password.make()) {
     Ok(_)      => { }
     Err(error) => { println!("{:?}", error); break } // channel disconnected, stop thread
    }//match verification_sender.send(password.make()) {

    if !password.incriment() {
     break;

    }//if !password.incriment() {
   }//loop {
  }//move || {
 )//std::thread::Builder::new().name("sender".to_string()).spawn(
}//fn channel_filling(chars: String, password_size: usize, verification_sender: crossbeam_channel::Sender<String>) -> Result<std::thread::JoinHandle<()>, std::io::Error> {

fn main() {
 if let Some(archive) = std::env::args().nth(1) {
  if let Some(chars) = std::env::args().nth(2) {
   if let Some(size_string) = std::env::args().nth(3) {
    if let Some(threads_string) = std::env::args().nth(4) {
     match size_string.parse::<usize>() {
      Ok(size_usize) => {
       match threads_string.parse::<usize>() {
        Ok(threads_usize) => {
         let (notification_sender, notification_receiver): (std::sync::mpsc::Sender<String>, std::sync::mpsc::Receiver<String>) = std::sync::mpsc::channel();

         let (verification_sender, verification_receiver): (crossbeam_channel::Sender<String>, crossbeam_channel::Receiver<String>) = crossbeam_channel::bounded(2000);

         match channel_filling(chars, size_usize, verification_sender) {
          Ok(thread_sender) => {
           let mut index: usize = 0;

           let mut threads_recipient: Vec<std::thread::JoinHandle<()>> = Vec::new();

           while index < threads_usize {
            match channel_emptying(archive.clone(), notification_sender.clone(), format!("thread_{}", index), verification_receiver.clone()) {
             Ok(thread_recipient) => {
              threads_recipient.push(thread_recipient);

             }//Ok(thread_recipient) => {

             Err(error) => { println!("match channel_emptying(archive.clone(), receiver.clone(), format!(thread, index)): {:?}", error); }
            }//match channel_emptying(archive.clone(), notification_sender.clone(), format!("thread_{}", index), verification_receiver.clone()) {

            index += 1; 
           }//while index < threads_usize {

           loop {
            match notification_receiver.try_recv() {
             Ok(password) => {
              println!("Password: {:?}", password);

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

              break;
             }//Ok(password) => {

             Err(std::sync::mpsc::TryRecvError::Disconnected) => {
              break;

             }//Err(std::sync::mpsc::TryRecvError::Disconnected) => {

             Err(std::sync::mpsc::TryRecvError::Empty) => {
              //Do nothing ...

             }//Err(std::sync::mpsc::TryRecvError::Empty) => {
            }//match notification_receiver.try_recv() {
           }//loop {
          }//Ok(thread_sender) => {

          Err(error) => { println!("channel_filling(passwords_creating(chars, length_u8), sender): {:?}", error); }
         }//match channel_filling(chars, size_usize, verification_sender) {
        }//Ok(threads_usize) => {

        Err(error) => { println!("match threads.parse::<u8>(): {:?}", error); }
       }//match threads_string.parse::<usize>() {
      }//Ok(size_usize) => {

      Err(error) => { println!("match length_string.parse::<u8>(): {:?}", error); }
     }//match size_string.parse::<usize>() {
    }//if let Some(threads_string) = std::env::args().nth(4) {
   }//if let Some(size_string) = std::env::args().nth(3) {
  }//if let Some(chars) = std::env::args().nth(2) {
 }//if let Some(archive) = std::env::args().nth(1) {
}//fn main() {
