
use rspotify::{model::{album, artist, playlist, user, AlbumId, Market, SearchResult, SearchType, SimplifiedPlaylist, TrackId, Type, UserId}, prelude::*, scopes, AuthCodePkceSpotify, Credentials, OAuth};


pub struct PlaySpotify{
    pub is_something_playing: bool, 
    pub spotify: AuthCodePkceSpotify,
}

impl PlaySpotify {

    pub fn new() -> Self{
        // Using env logger for extra debugging. 
        env_logger::init();

        // Get the already inputted credidentials form an env file.
        let creds = Credentials::from_env().unwrap();
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

        Self{
            is_something_playing: true,
            spotify: spotify,
        }
    }
    
    pub fn get_user_playlists(&mut self) -> Vec<SimplifiedPlaylist>{
        let mut user_playlists: Vec<SimplifiedPlaylist> = Vec::new();
        

        for playlist in self.spotify.current_user_playlists() {
            match playlist {
                Ok(data) => user_playlists.push(data), 
                Err(e) => println!("{:#?}", e)
            }
        }

        user_playlists
    }

    

}