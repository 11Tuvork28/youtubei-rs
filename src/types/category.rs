use super::{video::{Video}, search_channel::{SearchChannel}, search_playlist::SearchPlaylist, search_video::SearchVideo, channel_video::ChannelVideo};

pub enum CategoryTypes{
    Video(ChannelVideo),
    SearchChannel(SearchChannel),
    SearchPlaylist(SearchPlaylist),

}
pub struct Category{
   pub  title: String,
   pub  contents: Vec<CategoryTypes>,
   pub  description_html: String,
   pub  url: String,
   pub  badges: String,
}

impl Category {

}