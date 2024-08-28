
use std::fmt::Error;
//use anyhow::Ok;
use std::result::Result::Ok;

//use anyhow::Error; 
use rspotify::{model::{album, artist, playlist, track, user, AlbumId, ArtistId, FullArtist, Market, Page, SearchResult, SearchType, SimplifiedPlaylist, TrackId, Type, UserId}, prelude::*, scopes, AuthCodePkceSpotify, ClientError, Credentials, OAuth};


pub struct PlaySpotify<'a>{
    pub is_something_playing: bool, 
    pub spotify: AuthCodePkceSpotify,
    pub results_artist: Vec<artist_results<'a>>,
    pub results_album: Vec<album_reslults<'a>>,
    pub results_tracks: Vec<tracs_results<'a>>,
    
}

//#[derive(Debug)]
pub struct tracs_results<'a>{
    pub playable_id: PlayableId<'a>,
    pub name: String, 
    pub artist: String,
}
pub struct artist_results<'a>{
    pub name: String, 
    pub artist_id: ArtistId<'a>
    
}
pub struct album_reslults<'a>{
    pub album_name: String, 
    pub album_maker: String, 
    pub album_id: AlbumId<'a>
}

impl PlaySpotify <'_> {

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
            results_album: Vec::new(),
            results_artist: Vec::new(),
            results_tracks: Vec::new(),
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

    pub fn search_artist(&mut self, search_query: &str, limit:u32) -> Result<(), ClientError> {
        let seartch_spotify = self.spotify.search(&search_query, SearchType::Artist, None, None, Some(limit), None); 
       
        match seartch_spotify {
            Ok(SearchResult::Artists(artist_page))=>{
                //println!("this is the artist page {:#?}", artist_page);
                let mut  result_vec: Vec<artist_results> = Vec::new(); 
               
                for artist_page in artist_page.items{    
                    let result = artist_results {
                        name: artist_page.name, 
                        artist_id: artist_page.id,
                    };
                    self.results_artist.push(result);
                }
                Ok(())
            }
            Ok(_) =>{
                println!("not an artist");
                Ok(())
            }
            Err(e) => {
                println!("printing error {:?}", e); 
                Err(e)
            }
        }

    }

    pub fn search_album(&mut self, search_query: &str, limit:u32) -> Result<(), ClientError>{
        let seartch_spotify = self.spotify.search(&search_query, SearchType::Album, None, None, Some(limit), None); 
       
        match seartch_spotify {
            Ok(SearchResult::Albums(album_page))=>{
                //println!("this is the artist page {:#?}", album_page);                
                for album_page in album_page.items{   

                    let artist_name = album_page.artists.get(0)
                        .map(|artist| artist.name.clone())
                        .unwrap_or_else(|| "Unknown Artist".to_string());

                    let result = album_reslults {
                        album_name: album_page.name, 
                        album_maker: artist_name, 
                        album_id: album_page.id.expect("msg"),
                    };
                    self.results_album.push(result);
                }
                for rec in &self.results_album {
                   println!("Album Name: {}", rec.album_name);
                   println!("Album maker: {}", rec.album_maker);
                    // println!("Playable ID: {:?}", rec.playable_id); // Use {:?} if `playable_id` implements Debug
                    println!("------------------------------------");
                }
                Ok(())
            }
            Ok(_) =>{
                println!("not an artist");
                Ok(())
            }
            Err(e) => {
                println!("printing error {:?}", e); 
                Err(e)
            }
        }
    }
    
    pub fn search_tracks(&mut self, search_query: &str, limit:u32) -> Result<(), ClientError> {
        let seartch_spotify = self.spotify.search(&search_query, SearchType::Track, None, None, Some(limit), None); 
        match seartch_spotify {
            Ok(SearchResult::Tracks(tracks_page))=>{
                //println!("this is the tracks page {:#?}", tracks_page);
                for tracks_page in tracks_page.items{
                    let playable_id = PlayableId::Track(tracks_page.id.clone().expect("Track ID is None"));
                    // Extract the artist's name
                    let artist_name = tracks_page.artists.get(0)
                        .map(|artist| artist.name.clone())
                        .unwrap_or_else(|| "Unknown Artist".to_string());
                    
                    let result = tracs_results {
                        playable_id: playable_id, 
                        name: tracks_page.name, 
                        artist: artist_name, 
                    };
                    self.results_tracks.push(result);
                   // Iterate through and print each track result

                    
                    
                }
                for rec in &self.results_tracks {
                    println!("Track Name: {}", rec.name);
                    println!("Artist: {}", rec.artist);
                // println!("Playable ID: {:?}", rec.playable_id); // Use {:?} if `playable_id` implements Debug
                    println!("------------------------------------");
                }

                Ok(())
            },
            Ok(_) =>{
                println!("not a track");
                let result_vec: Vec<tracs_results> = Vec::new();
                Ok(())
            },
            Err(e) => {
                println!("printing error {:?}", e); 
                Err(e)
            }
        }
    }

    pub fn play_song(&mut self, song: usize){
        if let Some(first_playable_id) = self.results_tracks.get(song).and_then(|track|Some(track.playable_id.clone_static())){
            
            let playsong = self
                .spotify
                .start_uris_playback(vec![first_playable_id], None, None, None);

            match playsong {
                Ok(data) => println!("{:#?}", data),
                Err(e) => println!("Error: {:?}", e),
            }
        }else {
            println!("Error: Could not find a valid playable id for the selected track")
        }
    }
    pub fn play_album(&mut self, album: usize){
        if let Some(playable_album) = self.results_album.get(album).and_then(|album|Some(album.album_id.clone_static())){
            
            let playsong = self
                .spotify
                .start_context_playback(rspotify::prelude::PlayContextId::Album(playable_album), None, None, None);

            match playsong {
                Ok(data) => println!("{:#?}", data),
                Err(e) => println!("Error: {:?}", e),
            }
        }else {
            println!("Error: Could not find a valid playable id for the selected track")
        }
    }

    pub fn play_artist(&mut self, artist: usize){
        if let Some(playable_artist) = self.results_artist.get(artist).and_then(|artist|Some(artist.artist_id.clone_static())){
            
            let playsong = self
                .spotify
                .start_context_playback(rspotify::prelude::PlayContextId::Artist(playable_artist), None, None, None);

            match playsong {
                Ok(data) => println!("{:#?}", data),
                Err(e) => println!("Error: {:?}", e),
            }
        }else {
            println!("Error: Could not find a valid playable id for the selected track")
        }
    }
}

