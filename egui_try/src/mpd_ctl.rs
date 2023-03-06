use chrono::Duration;
use egui_extras::RetainedImage;
use mpd::Client;
use std::path::Path;

use std::fs::File;
use std::io::Read;

static MUSCI_DIRECTORY_PATH: &str = "/home/bido/.mpd/music/";

// conn.volume(4).unwrap();
// conn.play().unwrap();
// conn.single(true);

// Todo Life timize!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
pub fn album(conn: &mut Client) -> String {
    let song_pos = conn.status().unwrap().song.unwrap().pos;
    let song = conn.songs(song_pos).unwrap();

    song[0].tags["Album"].clone()
}

// Todo Life timize!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
pub fn title(conn: &mut Client) -> String {
    let song_pos = conn.status().unwrap().song.unwrap().pos;
    let song = conn.songs(song_pos).unwrap();

    song[0].title.as_ref().unwrap().clone()
}

// Todo Life timize!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
pub fn artist(conn: &mut Client) -> String {
    let song_pos = conn.status().unwrap().song.unwrap().pos;
    let song = conn.songs(song_pos).unwrap();

    song[0].tags["Artist"].clone()
}

pub fn progress(conn: &mut Client) -> f32 {
    let (elapsed_time, length) = conn.status().unwrap().time.unwrap();
    elapsed_time.num_seconds() as f32 / length.num_seconds() as f32
}
pub fn elapsed_time(conn: &mut Client) -> Duration {
    conn.status().unwrap().time.unwrap().0
}
pub fn song_length(conn: &mut Client) -> Duration {
    conn.status().unwrap().time.unwrap().1
}

/*
    Todo ちょっとパフォーマンスが悪すぎるのでどうにか考える
    どれか?
    1. とりあえず cargo bench する

    1. coverの先読み関数を実装する
    2. release profileでどうにかする
*/
pub fn cover(conn: &mut Client) -> RetainedImage {
    let song_pos = conn.status().unwrap().song.unwrap().pos;
    let song = &conn.songs(song_pos).unwrap()[0];

    let path = Path::new(&song.file).parent().unwrap();
    let cover_path = Path::new(MUSCI_DIRECTORY_PATH).join(path).join("cover.jpg");

    let mut cover_img = File::open(&cover_path).unwrap();
    let mut buffer = Vec::new();
    cover_img.read_to_end(&mut buffer).unwrap();

    RetainedImage::from_image_bytes(format!("{:?}", &cover_path), &buffer).unwrap()
}

#[cfg(test)]
mod tests {
    use super::cover;
    #[test]
    fn get_status() {
        let mut connection = mpd::Client::connect("127.0.0.1:6600").unwrap();
        let song_pos = connection.status().unwrap().song.unwrap().pos;
        let song = &connection.songs(song_pos).unwrap()[0];
    }

    #[test]
    fn run_cover() {
        let mut connection = mpd::Client::connect("127.0.0.1:6600").unwrap();
        cover(&mut connection);
    }
}
