use std::{borrow::Borrow, fs::File, io::Write, pin::Pin};

use futures::{future::{self, TryJoinAll}, Future, FutureExt, Stream, TryFutureExt};
use rusty_ytdl::{
    search::{Playlist, PlaylistSearchOptions},
    Video, VideoError,
};

use tokio::{
    sync,
    task::{self, JoinHandle},
};

// async fn fib_cpu_intensive(count : u8) -> Result<u8,()> {
//     thread::sleep(Duration::from_secs(5));
//     println!("Function {}", &count);
//     return Result::Ok(count);
// }

// #[tokio::main]
// async fn main(){
//     let mut futureFunctions : Vec<Result<u8, ()>> = Vec::new();
//     for i in 1..10{
//         futureFunctions.push(fib_cpu_intensive(i));
//     }
//     todo!()
// }

struct OurVid<'a> {
    vid: &'a Video,
    title: String,
}

#[tokio::main]
async fn main() {
    let playlist = Playlist::get(
        "https://www.youtube.com/playlist?list=PLewAP9xUTx3-UZWF65VNtADrMOQLwgAU1",
        Some(&PlaylistSearchOptions {
            limit: 9999,
            ..Default::default()
        }),
    )
    .await
    .unwrap();

    let mut test: Vec<Pin<Box<dyn Future<Output = Result<(), VideoError>> + Send>>> = Vec::new();
    for video in playlist.videos {
        test.push(Box::pin(download_bs(Video::new(video.url).unwrap(), video.title)));
    }

   let _ =  future::try_join_all(test.into_iter().map(tokio::spawn)).await;
}

async fn download_bs(video: Video, video_title : String) -> Result<(), VideoError> {
    let stream = video.stream().await.unwrap();
    let mut file = File::create(
        
        "downloads\\"
            .to_string()
            + &video_title
            + ".mp3",
    )
    .map_err(|e| VideoError::DownloadError(e.to_string()))?;

    while let Some(chunk) = stream.chunk().await.unwrap() {
        file.write_all(&chunk)
            .map_err(|e| VideoError::DownloadError(e.to_string()))?;
    }

    Ok(())
}
