Pop!_OS setup script coming soon!

// Setting up Pop!_OS System Config

pub mod PopOS {
   pub fn Hello(name:String) {
      println!("Hello {}",name);
   }
}

fn main(){
   PopOS::Hello("Pop!_OS users".to_string());
}
