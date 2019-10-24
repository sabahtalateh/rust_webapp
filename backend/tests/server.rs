//! The Server tests

#![cfg(test)]

use failure::Fallible;
use webapp::config::Config;
use backend::server::Server;
use url::Url;
use std::{thread, sync::Mutex, str::FromStr};
use reqwest::{Client, RedirectPolicy};
use lazy_static::lazy_static;
use http::StatusCode;

lazy_static! {
    static ref PORT: Mutex<u16> = Mutex::new(10101);
}

fn get_config() -> Fallible<Config> {
    Ok(Config::from_file("../Config.tests.toml")?)
}

fn next_port() -> u16 {
    let mut port = PORT.lock().unwrap();
    *port += 1;
    *port
}

fn create_test_server() -> Fallible<Config> {
    let conf = get_config()?;
    Ok(create_server_from_config(conf)?)
}

fn create_server_from_config(mut conf: Config) -> Fallible<Config> {
    let mut url = Url::parse(&conf.server.url)?;
    let _ = url.set_port(Some(next_port()));
    conf.server.url = url.to_string();

    let conf_cloned = conf.clone();

    // Start the server in different thread
    thread::spawn(move || Server::from_config(&conf_cloned).unwrap().start());

    // Wait until the server is up
    loop {
        if let Ok(res) = Client::new().get(url.as_str()).send() {
            if res.status().is_success() {
                break;
            }
        }
    }

    Ok(conf)
}

#[test]
fn succeed_to_create_server() -> Fallible<()> {
    let server_conf = create_test_server();
    assert!(server_conf.is_ok());

    let resp = Client::new().get(server_conf?.server.url.as_str()).send()?;
    assert!(resp.status().is_success());

    Ok(())
}

#[test]
fn redirects_succeed() -> Fallible<()> {
    let mut server_conf = get_config()?;

    // Add 2 redirect
    let mut redirect_from = Url::from_str(&server_conf.server.url)?;
    let _ = redirect_from.set_port(Some(next_port()));
    server_conf.server.redirect_from.push(redirect_from.to_string());

    let mut redirect_from = Url::from_str(&server_conf.server.url)?;
    let _ = redirect_from.set_port(Some(next_port()));
    server_conf.server.redirect_from.push(redirect_from.to_string());

    // Create server from config
    server_conf = create_server_from_config(server_conf.clone())?;

    let client = Client::builder()
        .redirect(RedirectPolicy::none())
        .build()?;

    server_conf.server.redirect_from.iter().for_each(|redirect_from| {
        // Check status is Permanent Redirect
        let res = client.get(redirect_from).send().unwrap();
        assert!(res.status() == StatusCode::PERMANENT_REDIRECT);

        // Check redirect url is the server url
        let redirect_to = Url::from_str(res.headers().get("location").unwrap().to_str().unwrap()).unwrap();
        let server_url = Url::from_str(&server_conf.server.url).unwrap();
        assert_eq!(redirect_to, server_url);
    });

    Ok(())
}
