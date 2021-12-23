// mod events {

//     fn events(tick_rate: Duration) -> mpsc::Receiver<Event> {
//         let (tx, rx) = mpsc::channel();
//         let keys_tx = tx.clone();
//         thread::spawn(move || {
//             let stdin = io::stdin();
//             for evt in stdin.keys() {
//                 if let Ok(key) = evt {
//                     if let Err(err) = keys_tx.send(Event::Input(key)) {
//                         eprintln!("{}", err);
//                         return;
//                     }
//                 }
//             }
//         });
//         thread::spawn(move || loop {
//             if let Err(err) = tx.send(Event::Tick) {
//                 eprintln!("{}", err);
//                 break;
//             }
//             thread::sleep(tick_rate);
//         });
//         rx
//     }