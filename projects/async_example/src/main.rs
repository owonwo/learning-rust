use futures::executor::block_on;
use futures::{join, Future};

fn main() {
    // let future = fetch_json();
    // block_on(future);

    // let song = block_on(learn_song());
    // block_on(sing_song(song));
    // block_on(dance());
    let song = get_song();

    block_on(async {
        // learn first, then sing but keep dancing while learning
        let f1 = learn_and_sing(&song);
        let f2 = dance();

        let (_a, _b) = join!(f1, f2,);

        println!("{_b}");
    });

    println!("Processing completed");
}


async fn learn_and_sing(song: &Song) {
    learn_song(&song).await;
    sing_song(&song).await;
}

async fn fetch_json() {
    println!("Hello, world!");
}

struct Song {
    title: String,
}


fn get_song() -> Song {
    Song { title: String::from("God is Good!") }
}

async fn learn_song(song: &Song) {
    println!("Learning song: {}", song.title)
}

async fn sing_song(song: &Song) {
    println!("Singing song: {}", song.title)
}

async fn dance() -> String {
    println!("Dancing to the song");

    String::from("Dance complete")
}