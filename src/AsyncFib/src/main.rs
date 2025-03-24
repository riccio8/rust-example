use tokio::task;

async fn fib(i: i64) -> i64 {
    match i {
        0 => 0,
        1 => 1,
        _ => {
            let f1 = task::spawn_blocking(move || fib_sync(i - 1));
            let f2 = task::spawn_blocking(move || fib_sync(i - 2));

            let r1 = f1.await.unwrap();
            let r2 = f2.await.unwrap();

            r1 + r2
        }
    }
}

fn fib_sync(i: i64) -> i64 {
    match i {
        0 => 0,
        1 => 1,
        _ => fib_sync(i - 1) + fib_sync(i - 2),
    }
}


#[tokio::main]
async fn main() {
    let n = fib(100).await;
    println!("{}", n);
}



// per misurare il tempo impiegato nell'eseguire codice:
// let start = std::time::Instant::now();
//quello che deve fare...();
//let end = std::time::instalnt::now();
//dbg!("Time {:#?}", end - start);

//fib non è effettivamente asincrona. Anche se dichiarato async fn fib(i: i32) -> i32, la funzione non sta facendo operazioni asincrone come await su qualcosa.
//   usare tokio::spawn_blocking per eseguire i calcoli in parallelo, non spawn e basta pk
//   fibonacci ha bisogno di cpu, non aspetta operazioni di I/O o di rete ma di cpu, cpu_bound,
//   quindi spawna un nuovo thread
  //  La funzione deve restituire un Future, quindi usare tokio::task::spawn per gestire le chiamate ricorsive in modo asincrono.
