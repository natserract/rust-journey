
use futures::future::join3;
use std::thread::sleep;
use std::time::Duration;

async fn hello() {
    let tent_secs = Duration::from_secs(1);
    sleep(tent_secs);
    println!("Bem")
}

async fn hello2() {
    let tent_secs = Duration::from_secs(5);
    sleep(tent_secs);
    println!("Bem2")
}

async fn hello3() {
    let tent_secs = Duration::from_secs(7);
    sleep(tent_secs);
    println!("Bem3")
}

pub async fn impl_of_async() {

    let a = hello();
    let b = hello2();
    let c = hello3();
    
    let tuple = join3(a, b, c);

    println!("-> {:?}", tuple.await);
}
