use std::fmt::Error;
use crate::endpoints;
use crate::endpoints::*;
use crate::extractors::*;
use crate::types::channel::{Channel,ChannelTab,Tab};
use crate::types::client::ClientConfig;
use crate::types::playlist::Playlist;
use crate::types::query_results::{CommentsQuery, VideoQuery, ChannelQuery,SearchQuery};
use crate::types::video::{SearchVideo,Video};

pub async fn search(query: String,client_config: &ClientConfig) -> Result<SearchQuery, Error>{
    let json = endpoints::search(&query, "", &client_config).await;
    if !json["error"].is_null(){
        panic!("Unexpected error: {}", json["error"].to_string());
    }
    return Ok(extract_search_results(&json, false));
}

pub async fn load_search(continuation:String,client_config: &ClientConfig) ->Result<SearchQuery, Error>{
    let json = endpoints::search_continuation(&continuation, &client_config).await;
    if !json["error"].is_null(){
        panic!("Unexpected error: {}", json["error"].to_string());
    }
    return Ok(extract_search_results(&json, true));
}
pub async fn load_related_videos(continuation:String,client_config: &ClientConfig) -> Result<Vec<SearchVideo>, Error>{
    let json = next(&continuation, client_config).await;
    if !json["error"].is_null(){
        panic!("Unexpected error: {}", json["error"].to_string());
    }
    Ok(load_related(&json))
}

pub async fn get_comments(continuation:String,client_config: &ClientConfig) ->Result<CommentsQuery,  Error>{
    let comments_json = next(&continuation, client_config).await;
    if !comments_json["error"].is_null() || comments_json["onResponseReceivedEndpoints"].is_null(){
        panic!("Wrong token!");
    }
    Ok(extract_comments(&comments_json))
}

pub async fn get_video(video_id:String, params: String,client_config: &ClientConfig) ->Result<VideoQuery,  Error>{
    let player_json = player(&video_id, &params, &client_config).await;
    /*
    Error handling
    */
    if player_json["playabilityStatus"]["status"].as_str().unwrap() == "ERROR" || !player_json["error"].is_null() {
        panic!("{}", player_json["playabilityStatus"]["reason"].as_str().unwrap());
    }
    let next_video_data = next_with_data(serde_json::json!({
        "videoId": video_id,
        "params": params 
    }),&client_config).await;
    let video_player = extract_video_player_formats(&player_json["streamingData"]);
    let video: Video = video_from_next_and_player(&player_json, &next_video_data["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"], video_player);
    Ok(extract_next_video_results(&next_video_data, VideoQuery{
        continuation_comments: "".to_string(),
        continuation_related: next_video_data["contents"]["twoColumnWatchNextResults"]["secondaryResults"]["secondaryResults"]["results"][20]["continuationItemRenderer"]["continuationEndpoint"]["continuationCommand"]["token"].to_string(),
        video,
        related_videos: Vec::new(),
    }))
}

pub async fn get_channel_info(url:String,client_config: &ClientConfig) -> Result<ChannelQuery,  Error>{
    let complete_url = url.to_string()+"/about"; 
    let resolved_url = resolve_url(&complete_url,&client_config ).await;
    if !resolved_url["error"].is_null(){
        panic!("{}",resolved_url["error"]["message"]);
    }
    let channel_json = browse_browseid(
        resolved_url["endpoint"]["browseEndpoint"]["browseId"].as_str().unwrap(), 
        resolved_url["endpoint"]["browseEndpoint"]["params"].as_str().unwrap(), 
        &client_config
    ).await;
    let channel: Channel = extract_channel_info(&channel_json);
    Ok(ChannelQuery{
        channel,
    })
}
pub async fn get_channel_tab_url(url:String,tab: Tab, client_config: &ClientConfig) -> Result<ChannelTab, Error>{
    let index = tab.get_index();
    let complete_url = url + "/"+ tab.get_name();
    let resolved_url = resolve_url(&complete_url,&client_config).await;
    if !resolved_url["error"].is_null(){
        panic!("{}",resolved_url["error"]["message"]);
    }
    let channel_json = browse_browseid(
        resolved_url["endpoint"]["browseEndpoint"]["browseId"].as_str().unwrap(), 
        resolved_url["endpoint"]["browseEndpoint"]["params"].as_str().unwrap(), 
        &client_config
    ).await;
    Ok(extract_channel_tab(&channel_json,index))
}
pub async fn get_channel_tab_continuation(continuation:String,tab: Tab, client_config: &ClientConfig) -> Result<ChannelTab, Error>{
    let index = tab.get_index();
    let channel_json = browse_continuation(&continuation,&client_config).await;
    Ok(extract_channel_tab(&channel_json,index))
}
pub async fn get_playlist(playlist_id: String,client_config: &ClientConfig)-> Result<Playlist, Error>{
    let complete_url = "https://www.youtube.com/playlist?list=".to_owned()+ &playlist_id;
    let resolved_url = resolve_url(&complete_url,&client_config).await;
    if !resolved_url["error"].is_null(){
        panic!("{}",resolved_url["error"]["message"]);
    }
    let playlist_json = browse_browseid(
        resolved_url["endpoint"]["browseEndpoint"]["browseId"].as_str().unwrap(), 
        "", 
        &client_config
    ).await;
    Ok(extract_playlist(&playlist_json))
}