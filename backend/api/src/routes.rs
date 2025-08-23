use actix_web::web;
use actix_web::web::scope;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::handlers::affiliated_artists::config as AffiliatedArtistsConfig;
use crate::handlers::artists::config as ArtistsConfig;
use crate::handlers::auth::config as AuthConfig;
use crate::handlers::conversations::config as ConversationsConfig;
use crate::handlers::edition_groups::config as EditionGroupsConfig;
use crate::handlers::external_db::config as ExternalDbConfig;
use crate::handlers::forum::config as ForumConfig;
use crate::handlers::search::config as SearchConfig;
use crate::handlers::series::config as SeriesConfig;
use crate::handlers::subscriptions::config as SubscriptionsConfig;
use crate::handlers::title_groups::config as TitleGroupsConfig;
use crate::handlers::torrent_requests::config as TorrentRequestsConfig;
use crate::handlers::torrents::config as TorrentsConfig;
use crate::handlers::user_applications::config as UserApplicationsConfig;
use crate::handlers::users::config as UsersConfig;

use crate::handlers::{
    announce_handler::handle_announce,
    gift_handler::send_gift,
    home_handler::get_home,
    invitation_handler::send_invitation,
    master_group_handler::add_master_group,
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
            .service(scope("/series").configure(SeriesConfig))
            .service(scope("/external_db").configure(ExternalDbConfig))
            .service(scope("/forum").configure(ForumConfig))
            .route("/home", web::get().to(get_home))
            .route("/invitation", web::post().to(send_invitation))
            .route("/master-group", web::post().to(add_master_group))
            .route("/report/torrent", web::post().to(add_torrent_report))
            .route("/gift", web::post().to(send_gift))
            .route("/wiki/article", web::post().to(add_wiki_article))
            .route("/wiki/article", web::get().to(get_wiki_article)),
    );
}
