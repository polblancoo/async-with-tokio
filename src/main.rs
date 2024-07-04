use tokio::runtime::{Handle, RuntimeFlavor};


#[tokio::main(flavor = "multi_thread")]
async fn main() {
    //test_something().await;
    let racer01 = F1Racer::new();
    let mut racer02 = F1Racer::new();
    
    racer02.name= "Sergio Perez".to_string();
    racer02.lap_times.pop();
    racer02.lap_times.push(57);


    
  let handle01 = tokio::task::spawn(racer01);
  let handle02 =tokio::task::spawn(racer02);
    
  loop{

    if handle01.is_finished() && handle02.is_finished() {
        println!("All racer have finished !");
        break;
    }

    std::thread::sleep(std::time::Duration::from_millis(300));

  }  


 }

 /* async fn test_something(){
    std::thread::sleep(std::time::Duration::from_millis(5000));
    println!("Hello from tokio");
 } */
  struct F1Racer{
    name: String,
    complete_laps: u8,
    laps: u8,
    best_lap_time: u8,
    lap_times: Vec<u8>,
  }
  impl F1Racer{
    fn new()-> F1Racer{

        F1Racer{
        name:"Max Verstappen".to_string(),
        laps: 5,
        complete_laps:0,
        best_lap_time: 255 ,
        lap_times: vec![87u8, 64, 125, 95 ,76]    
        }
    }
    fn do_lap(&mut self){
        println!(" {} is doing a new lap...{}" , self.name ,self.complete_laps);
        let lap_time = self.lap_times.pop();
        if  lap_time.is_some() && lap_time.unwrap() < self.best_lap_time{
            self.best_lap_time = lap_time.unwrap();
        }
        self.complete_laps += 1;


    }

  }

  impl std::future::Future for F1Racer{
        type Output= u8;
        fn poll( self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
          println!("Thread assigned is ID: {:?}" , std::thread::current().id());  
          if self.complete_laps < self.laps {
            self.get_mut().do_lap();
            cx.waker().wake_by_ref();
            return std::task::Poll::Pending;
          }
            println!("{} has completed all laps !", self.name );
            println!("Best lap time for {} was : {}" ,self.name, self.best_lap_time);
    
            return std::task::Poll::Ready(self.best_lap_time);
        }

  }