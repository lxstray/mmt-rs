use yt_dlp::Downloader;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let executables_dir = PathBuf::from("libs");
    let output_dir = PathBuf::from("output");

    let downloader = Downloader::with_new_binaries(executables_dir, output_dir).await?.build().await?;

    let youtube = downloader.youtube_extractor();

    let url = String::from("https://www.youtube.com/watch?v=mgjZCs1lcxY");
    let video = downloader.fetch_video_infos(url).await?;
    let video_path = downloader.download_video(&video, "my-video.mp4").await?;

    Ok(())

}
