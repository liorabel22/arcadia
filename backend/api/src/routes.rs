use actix_web::web;
use actix_web::web::scope;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::handlers::auth::config as AuthConfig;
use crate::handlers::edition_groups::config as EditionGroupsConfig;
use crate::handlers::search::config as SearchConfig;
use crate::handlers::title_groups::config as TitleGroupsConfig;
use crate::handlers::torrents::config as TorrentsConfig;
use crate::handlers::user_applications::config as UserApplicationsConfig;
use crate::handlers::users::config as UsersConfig;

use crate::handlers::torrent_request_handler::search_torrent_requests;
use crate::handlers::{
    announce_handler::handle_announce,
    artist_handler::{
        add_affiliated_artists, add_artists, get_artist_publications, get_artists_lite,
        remove_affiliated_artists,
    },
    conversation_handler::{
        add_conversation, add_conversation_message, get_conversation, get_user_conversations,
    },
    forum_handler::{
        add_forum_post, add_forum_thread, get_forum, get_forum_sub_category_threads,
        get_forum_thread, search_forum_thread,
    },
    gift_handler::send_gift,
    home_handler::get_home,
    invitation_handler::send_invitation,
    master_group_handler::add_master_group,
    scrapers::{
        comic_vine::get_comic_vine_data, isbn::get_isbn_data, musicbrainz::get_musicbrainz_data,
        tmdb::get_tmdb_data,
    },
    series_handler::{add_series, get_series},
    subscriptions_handler::{add_subscription, remove_subscription},
    torrent_report_handler::add_torrent_report,
    torrent_request_handler::{add_torrent_request, fill_torrent_request, get_torrent_request},
    torrent_request_vote_handler::add_torrent_request_vote,
    wiki_handler::{add_wiki_article, get_wiki_article},
};
use crate::middlewares::jwt_middleware::authenticate_user;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(handle_announce).service(
        web::scope("/api")
            .wrap(HttpAuthentication::with_fn(authenticate_user))
            .service(scope("/auth").configure(AuthConfig))
            .service(scope("/users").configure(UsersConfig))
            .service(scope("/user-applications").configure(UserApplicationsConfig))
            .service(scope("/title-groups").configure(TitleGroupsConfig))
            .service(scope("/edition-groups").configure(EditionGroupsConfig))
            .service(scope("/search").configure(SearchConfig))
            .service(scope("/torrents").configure(TorrentsConfig))
            .route("/home", web::get().to(get_home))
            .route("/invitation", web::post().to(send_invitation))
            .route("/master-group", web::post().to(add_master_group))
            .route("/report/torrent", web::post().to(add_torrent_report))
            .route("/search/artist/lite", web::get().to(get_artists_lite))
            .route("/artists", web::post().to(add_artists))
            .route("/artist", web::get().to(get_artist_publications))
            .route(
                "/affiliated-artists",
                web::post().to(add_affiliated_artists),
            )
            .route(
                "/affiliated-artists",
                web::delete().to(remove_affiliated_artists),
            )
            .route("/series", web::post().to(add_series))
            .route("/series", web::get().to(get_series))
            .route("/torrent-request", web::post().to(add_torrent_request))
            .route("/torrent-request", web::get().to(get_torrent_request))
            .route(
                "/torrent-request/fill",
                web::post().to(fill_torrent_request),
            )
            .route(
                "/torrent-request/vote",
                web::post().to(add_torrent_request_vote),
            )
            .route(
                "/search/torrent-request",
                web::get().to(search_torrent_requests),
            )
            .route("/subscription", web::post().to(add_subscription))
            .route("/subscription", web::delete().to(remove_subscription))
            .route("/gift", web::post().to(send_gift))
            .route("/forum", web::get().to(get_forum))
            .route(
                "/forum/sub-category",
                web::get().to(get_forum_sub_category_threads),
            )
            .route("/forum/thread", web::get().to(get_forum_thread))
            .route("/search/forum/thread", web::get().to(search_forum_thread))
            .route("/forum/thread", web::post().to(add_forum_thread))
            .route("/forum/post", web::post().to(add_forum_post))
            .route("/wiki/article", web::post().to(add_wiki_article))
            .route("/wiki/article", web::get().to(get_wiki_article))
            .route("/conversation", web::post().to(add_conversation))
            .route("/conversation", web::get().to(get_conversation))
            .route("/conversations", web::get().to(get_user_conversations))
            .route(
                "/conversation/message",
                web::post().to(add_conversation_message),
            )
            .route("/external_db/isbn", web::get().to(get_isbn_data))
            .route("/external_db/tmdb", web::get().to(get_tmdb_data))
            .route("/external_db/isbn", web::get().to(get_isbn_data))
            .route(
                "/external_db/comic_vine",
                web::get().to(get_comic_vine_data),
            )
            .route(
                "/external_db/musicbrainz",
                web::get().to(get_musicbrainz_data),
            ),
    );
}
