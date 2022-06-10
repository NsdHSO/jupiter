#![allow(unused_variables)]

fn main() -> Result < () , Box<dyn Error>>{
   let mut audio = Audio::new();
   audio.add("bigExplode", "sound/bikfoot4.wav" );
   audio.add("startup", "sound/start.wav");
   audio.play("startup");

   audio.wait();
   OK(())
}
