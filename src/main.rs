mod life;
mod life_image;

use std::io::Read;
use std::sync::{Arc, Mutex};
use std::time;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
    routing::get,
    Router,
};
use tokio::sync::broadcast;

use futures::{
    sink::SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};

use life::Life;
use life_image::draw_image_data_url;

// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main1() {
    life::start_life(
        // width, height
        // 7, 3,
        80,
        38,
        // 230, 100,
        // 5_000, 5_000,
        // init shape
        // life::Shape::Empty,
        life::Shape::Random,
        // life::Shape::Glider,
        // Worker threads
        1,
        // max iter
        10_000,
        // sleep ms
        120,
        // debug, show time per tick insted of board
        false,
    );
}

struct AppState {
    // We require unique usernames. This tracks which usernames have been taken.
    // user_set: Mutex<HashSet<String>>,
    // Channel used to send messages to all connected clients.
    tx: broadcast::Sender<String>,
}

// Based on:
// @see: https://github.com/tokio-rs/axum/blob/main/examples/chat/src/main.rs
#[tokio::main]
async fn main() {
    // let connections = Vec::<WebSocket>::new();
    // let cons = Arc::new(Mutex::new(connections));

    let (tx, _rx) = broadcast::channel(100);

    let app_state = Arc::new(AppState { tx });
    let state = app_state.clone();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/ws",
            get(
                |ws: WebSocketUpgrade, State(state): State<Arc<AppState>>| async {
                    return ws.on_upgrade(|socket| websocket_handler(socket, state));
                },
            ),
        )
        .with_state(app_state);

    // let mut rx_chan = state.tx.subscribe();

    // // Monitor channel.
    // tokio::spawn(async move {
    //     while let Ok(msg) = rx_chan.recv().await {
    //         println!("RX: {msg}");
    //     }
    // });

    let mut tx_chan = state.tx.clone();

    // Life.
    tokio::spawn(async move {
        // params
        let (width, height, init) = (5, 5, life::Shape::Blinker);
        // let (width, height, init) = (100, 100, life::Shape::Random);
        let n_workers = 1;
        let limit = 10_000;
        let wait = 1_000;
        let debug = false;
        // END params

        let sleep_time = time::Duration::from_millis(wait);
        let mut now;
        let mut game;

        println!("\n\nConway's Game of Life\n");
        println!("Board:    {}x{}", width, height);
        println!("Cells:    {}", width * height);
        println!("Workers:  {}", n_workers);
        println!("Max iter: {}", limit);
        println!("Wait:     {}ms", wait);

        println!("\nStarting ...\n");

        if debug {
            now = time::SystemTime::now();
            game = Life::new(width, height, init, n_workers);
            println!("Generating Game ! {:?}\n", now.elapsed());
        } else {
            game = Life::new(width, height, init, n_workers);
            // draw(&game.state);
        }

        let mut ticker = tokio::time::interval(sleep_time);

        loop {
            ticker.tick().await;

            if debug {
                now = time::SystemTime::now();

                game.tick();

                println!("Tick ! {:?}", now.elapsed());

                // let msg = format!("Tick ! {:?}", now.elapsed());

                // let _ = tx_chan.send(msg);

                let b = draw_image_data_url(&game.state);

                // let _ = tx_chan.send(b);

                match tx_chan.send(b) {
                    Err(err) => println!("Send error {:?}", err),
                    Ok(size) => println!("Send size {:?}", size),
                }

                // println!("IMG {:?}", b);
            } else {
                game.tick();
                let b = draw_image_data_url(&game.state);
                let _ = tx_chan.send(b);
            }
        }
    });

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn websocket_handler(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();

    let mut rx = state.tx.subscribe();

    if sender.send(Message::Text("HELLO".into())).await.is_err() {
        return;
    }

    let _ = state.tx.send("JOINED: todo".into());

    // Spawn the first task that will receive broadcast messages and send text
    // messages over the websocket to our client.
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // In any websocket error, break loop.
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let tx = state.tx.clone();

    // Spawn a task that takes messages from the websocket, prepends the user
    // name, and sends them to all broadcast subscribers.
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            // Add username before message.
            let _ = tx.send(format!("todo: {text}"));
        }
    });

    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }

    let _ = state.tx.send("LEFT: todo".into());
}
