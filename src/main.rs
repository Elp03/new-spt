#![allow(clippy::assigning_clones)]

use rspotify::{model::{playlist, user, SimplifiedPlaylist, Type, UserId}, prelude::*, scopes, AuthCodePkceSpotify, Credentials, OAuth};

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

    println!("This is current users playlists {:?}", user_playlists); 
}