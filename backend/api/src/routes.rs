use actix_web::web;
use actix_web::web::scope;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::handlers::affiliated_artists::config as AffiliatedArtistsConfig;
use crate::handlers::artists::config as ArtistsConfig;
use crate::handlers::auth::config as AuthConfig;
use crate::handlers::conversations::config as ConversationsConfig;
use crate::handlers::edition_groups::config as EditionGroupsConfig;
use crate::handlers::search::config as SearchConfig;
use crate::handlers::subscriptions::config as SubscriptionsConfig;
use crate::handlers::title_groups::config as TitleGroupsConfig;
use crate::handlers::torrent_requests::config as TorrentRequestsConfig;
use crate::handlers::torrents::config as TorrentsConfig;
use crate::handlers::user_applications::config as UserApplicationsConfig;
use crate::handlers::users::config as UsersConfig;

use crate::handlers::{
    announce_handler::handle_announce,
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
    torrent_report_handler::add_torrent_report,
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
            .service(scope("/torrent-requests").configure(TorrentRequestsConfig))
            .service(scope("/artists").configure(ArtistsConfig))
            .service(scope("/affiliated-artists").configure(AffiliatedArtistsConfig))
            .service(scope("/conversations").configure(ConversationsConfig))
            .service(scope("/subscriptions").configure(SubscriptionsConfig))
            .route("/home", web::get().to(get_home))
            .route("/invitation", web::post().to(send_invitation))
            .route("/master-group", web::post().to(add_master_group))
            .route("/report/torrent", web::post().to(add_torrent_report))
            .route("/series", web::post().to(add_series))
            .route("/series", web::get().to(get_series))
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
