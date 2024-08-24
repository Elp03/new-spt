#![allow(clippy::assigning_clones)]

use rspotify::{model::{artist, playlist, user, Market, SearchResult, SearchType, SimplifiedPlaylist, Type, UserId}, prelude::*, scopes, AuthCodePkceSpotify, Credentials, OAuth};

#[tokio::main]
async fn main() {
    // You can use any logger for debugging.
    env_logger::init();

    // Set RSPOTIFY_CLIENT_ID and RSPOTIFY_CLIENT_SECRET in an .env file (after
    // enabling the `env-file` feature) or export them manually:
    //
    // export RSPOTIFY_CLIENT_ID="your client_id"
    //
    // It will then be read with `from_env`.
    //
    // Otherwise, set client_id explictly:
    //
    // ```
    // let creds = Credentials::new_pkce("my-client-id");
    // ```

    let creds = Credentials::from_env().unwrap();

    // Same for RSPOTIFY_REDIRECT_URI. You can also set it explictly:
    //
    // ```
    // let oauth = OAuth {
    //     redirect_uri: "http://localhost:8888/callback".to_string(),
    //     scopes: scopes!("user-read-recently-played"),
    //     ..Default::default(),
    // };
    // ```

    let oauth = OAuth::from_env(scopes!("user-read-playback-state")).unwrap();

    let mut spotify = AuthCodePkceSpotify::new(creds.clone(), oauth.clone());

    // Obtaining the access token
    let url = spotify.get_authorize_url(None).unwrap();
    
    println!(" after acces token");
    // This function requires the `cli` feature enabled.
    spotify.prompt_for_token(&url).unwrap();

    // Running the requests
    let history = spotify.current_playback(None, None::<Vec<_>>);
    println!("Response: {history:?}");

    // Token refreshing works as well, but only with the one generated in the
    // previous request (they actually expire, unlike the regular code auth
    // flow).
    let prev_token = spotify.token.lock().unwrap();
    let spotify = AuthCodePkceSpotify::new(creds, oauth);
    *spotify.token.lock().unwrap() = prev_token.clone();
    spotify.refresh_token().unwrap();

    // Running the requests again
    let history = spotify.current_playback(None, None::<Vec<_>>);
    println!("Response after refreshing token: {history:?}");

    let mut user_playlists: Vec<SimplifiedPlaylist> = Vec::new();
    //let user_playlist = *spotify.current_user_playlists();
    for playlist in spotify.current_user_playlists() {
        match playlist {
            Ok(data) => user_playlists.push(data), 
            Err(e) => println!("{:#?}", e)
        }
    }
   // println!("This is current users playlists {:?}", user_playlists); 
   
    let search_query = "Paleale royal"; 
    let mut search_result: Vec<SearchResult> = Vec::new();

    let seartch_spotify = spotify.search(&search_query, SearchType::Artist, None, None, Some(1), None); 
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
        },
        Ok(SearchResult::Tracks(tracks)) => {
            println!("Tracks found: {}", tracks.items.len());
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


    println!("This is search {:?}", search_result); 

}