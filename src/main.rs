#![allow(clippy::assigning_clones)]

use std::any::Any;

use rspotify::{model::{album, artist, playlist, user, AlbumId, Market, SearchResult, SearchType, SimplifiedPlaylist, TrackId, Type, UserId}, prelude::*, scopes, AuthCodePkceSpotify, Credentials, OAuth};
use spotify::PlaySpotify;


mod spotify; 

#[tokio::main]
async fn main() {
    let mut play_spotify: PlaySpotify = PlaySpotify::new(); 
    //let mut spotify = play_spotify.spotify; 

    // Running the requests again
   // let history = spotify.current_playback(None, None::<Vec<_>>);
    //println!("Response after refreshing token: {history:?}");

    let mut user_playlists: Vec<SimplifiedPlaylist> = Vec::new();
    //let user_playlist = *spotify.current_user_playlists();
    // for playlist in spotify.current_user_playlists() {
    //     match playlist {
    //         Ok(data) => user_playlists.push(data), 
    //         Err(e) => println!("{:#?}", e)
    //     }
    // }
    user_playlists = play_spotify.get_user_playlists(); 
    println!("Here is user playlist {:#?}", user_playlists);

   // println!("This is current users playlists {:?}", user_playlists); 
   
    let search_query = "This is palaye royal"; 
    let mut search_result: Vec<SearchResult> = Vec::new();
    let track_uri;
    let seartch_spotify = spotify.search(&search_query, SearchType::Album, None, None, Some(3), None); 
    let play_this_playlist: PlayContextId;  

   // #[derive(Debug)]
    let playable_id: PlayableId; 

    match seartch_spotify {
        Ok(SearchResult::Artists(artist_page)) => {
            if let Some(artist) = artist_page.items.get(0) {
                println!("Artist found: {}", artist.name);
                println!("Popularity: {}", artist.popularity);
                println!("Followers: {}", artist.followers.total);
                println!("Genres: {:?}", artist.genres);
            } else {
                println!("No artist found with the name '{}'", search_query);
            }
        },
        Ok(SearchResult::Albums(albums)) => {
            println!("Albums found: {}", albums.items.len());
            println!("Album: {:#?}", albums);
            let album_test: Vec<AlbumId> = Vec::new(); 

            // for playlist in albums.items{
            //     let foo = playlist.artists;
            //     let foo_bar = &playlist.id; 
            //     //let bar = &foo[0].id;

            //     println!("here is the foobar {:#?}", foo_bar); 
            // }
            
           // let something = &albums.items[0].id;
           play_this_playlist = PlayContextId::Album(albums.items[0].id.clone().expect("Album is none"));
            
            
           // play_this_playlist = PlayableId::Album(something.clone().expect("msg"));

            spotify.start_context_playback(play_this_playlist, None, None, None); 

        },
        Ok(SearchResult::Tracks(tracks)) => {
            if let Some(tracks) = tracks.items.first() {
                track_uri = &tracks.id; 
                playable_id = PlayableId::Track(track_uri.clone().expect("msg"));
                //track_uri = TrackId::from_id(&tracks.id).unwrap().uri();
                println!("Track found: {} - URI: {:?}", tracks.name, playable_id.uri());
                
                let playsong = spotify.start_uris_playback(vec![playable_id], None, None, None);
                match playsong {
                    Ok(data) => println!("{:#?}", data),
                    Err(e) => println!("error {:?}", e),
                }
                // Now you can use this URI to play the track or for other purposes
            } else {
                println!("No track found for");
            }
        
        },
        Ok(SearchResult::Playlists(playlists)) => {
            println!("Playlists found: {}", playlists.items.len());
        },
        Ok(SearchResult::Shows(shows)) => {
            println!("Shows found: {}", shows.items.len());
        },
        Ok(SearchResult::Episodes(episodes)) => {
            println!("Episodes found: {}", episodes.items.len());
        },
        Err(e) => println!("{:#?}", e)
    }
    
    // for playlist in spotify.search(&search_query, SearchType::Artist, None, None, Some(1), None) {
    //     match playlist {
    //         Ok(data) => search_result.push(data), 
    //         Err(e) => println!("{:#?}", e)
    //     }
    // }

//    spotify.start_uris_playback(vec![playable_id], None, None, None);

}