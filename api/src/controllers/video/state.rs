use marco_polo_rs_core::internals::{
    cloud::traits::BucketClient, yt_downloader::traits::YoutubeDownloader,
};

pub struct State<YD, BC>
where
    YD: YoutubeDownloader,
    BC: BucketClient,
{
    pub youtube_downloader: YD,
    pub bucket_client: BC,
}
