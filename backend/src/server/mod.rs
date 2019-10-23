use actix::{SyncArbiter, SystemRunner};
use actix_cors::Cors;
use actix_web::{http::header::{CONTENT_TYPE, LOCATION}, middleware, App, HttpServer, HttpResponse};
use diesel::{r2d2::ConnectionManager, PgConnection};
use failure::{format_err, Fallible};
use r2d2::Pool;
use std::{net::{SocketAddr, ToSocketAddrs}, slice::from_ref};
use url::Url;
use webapp::config::Config;

use crate::database::DatabaseExecutor;
use openssl::ssl::{SslAcceptorBuilder, SslAcceptor, SslMethod, SslFiletype};
use actix_web::web::get;

pub struct Server {
    config: Config,
    runner: SystemRunner,
    url: Url,
}

fn resp() -> HttpResponse {
    HttpResponse::Ok().body("zozo")
}

impl Server {
    pub fn from_config(config: &Config) -> Fallible<Server> {
        let runner = actix::System::new("backend");

        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            config.postgres.username,
            config.postgres.password,
            config.postgres.host,
            config.postgres.port,
            config.postgres.database,
        );

        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().build(manager)?;
        let db_addr = SyncArbiter::start(num_cpus::get(), move || DatabaseExecutor(pool.clone()));

        let app = move || {
            App::new()
                .data(db_addr.clone())
                .wrap(
                    Cors::new()
                        .allowed_methods(vec!["OPTIONS", "GET", "POST"])
                        .allowed_header(CONTENT_TYPE)
                        .max_age(3600),
                )
                .wrap(middleware::Logger::default())
                .route("/zozoz", get().to(resp))
        };

        let server = HttpServer::new(app);

        let url = Url::parse(&config.server.url)?;
        let addrs = Self::url_to_socket_addrs(&url)?;
        if url.scheme() == "https" {
            server.bind_ssl(addrs.as_slice(), Self::build_tls(&config)?)?.start();
        } else {
            server.bind(addrs.as_slice())?.start();
        }

        Ok(Server {
            config: config.to_owned(),
            runner,
            url,
        })
    }

    pub fn start(self) -> Fallible<()> {
        self.runner.run()?;

        Ok(())
    }

    pub fn build_tls(config: &Config) -> Fallible<SslAcceptorBuilder> {
        let mut tls_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
        tls_builder.set_private_key_file(&config.server.key, SslFiletype::PEM);
        tls_builder.set_certificate_chain_file(&config.server.cert)?;
        Ok(tls_builder)
    }

    pub fn url_to_socket_addrs(url: &Url) -> Fallible<Vec<SocketAddr>> {
        let host = url
            .host()
            .ok_or_else(|| format_err!("No host name in the URL"))?;
        let port = url.port().ok_or_else(|| format_err!("No port in the URL"))?;

        let addrs;
        let addr;
        Ok(match host {
            url::Host::Domain(domain) => {
                addrs = (domain, port).to_socket_addrs()?;
                addrs.as_slice().to_owned()
            },
            url::Host::Ipv4(ip) => {
                addr = (ip, port).into();
                from_ref(&addr).to_owned()
            },
            url::Host::Ipv6(ip) => {
                addr = (ip, port).into();
                from_ref(&addr).to_owned()
            },
        })
    }
}
