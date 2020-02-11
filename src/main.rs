#[macro_use]
extern crate lazy_static;

use std::{env, time};

use iron::modifiers::Redirect;
use iron::prelude::*;
use iron::request::Url;
use iron::status;
use redis;
use router::Router;

fn tick(r: redis::Client, kit: String) {
    match r.get_connection() {
        Ok(mut r) => match time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH) {
            Ok(ts) => {
                let x: redis::RedisResult<()> = redis::cmd("HINCRBY")
                    .arg(format!("tick:{}", kit))
                    .arg("all")
                    .arg(1)
                    .query(&mut r);
                match x {
                    Ok(_) => (),
                    Err(e) => print!("failed to incr tick:{} -- {}\n", kit, e),
                };

                let ts = ts.as_secs();
                let x: redis::RedisResult<()> = redis::cmd("HINCRBY")
                    .arg(format!("tick:{}", kit))
                    .arg(format!("{}", ts - ts % 3600))
                    .arg(1)
                    .query(&mut r);
                match x {
                    Ok(_) => (),
                    Err(e) => print!("failed to incr tick:{} -- {}\n", kit, e),
                };
            }
            Err(e) => {
                print!("error: {}\n", e);
            }
        },
        Err(e) => {
            print!("error: {}\n", e);
        }
    };
}

lazy_static! {
    static ref REDIS: String = {
        match env::var("REDIS") {
            Ok(v) => v,
            _ => String::from("redis://127.0.0.1:6379"),
        }
    };
}

fn main() {
    let mut router = Router::new();

    router.get(
        "/:kit/:v/:scheme/*url",
        |r: &mut Request| {
            macro_rules! param {
                ($e:expr) => {
                    match r.extensions.get::<Router>() {
                        Some(e) => match e.find($e) {
                            Some(v) => v.to_string(),
                            _ => "".to_string(),
                        },
                        _ => "".to_string(),
                    }
                };
            }
            match redis::Client::open(REDIS.as_str()) {
                Ok(client) => tick(client, format!("{}/{}", param!("kit"), param!("v"))),
                Err(e) => print!("unable to connect to redis at {}: {}", REDIS.as_str(), e),
            }

            let url = format!("{}://{}", param!("scheme"), param!("url"));
            match Url::parse(&url) {
                Ok(url) => {
                    let mut res = Response::new();
                    res.set_mut(status::Found).set_mut(Redirect(url));
                    Ok(res)
                }
                Err(_) => Ok(Response::with((
                    status::BadRequest,
                    format!("400 Bad Request\n"),
                ))),
            }
        },
        "redirector",
    );

    let bind = "0.0.0.0:3000";
    println!("tracker starting up on {}", bind);
    Iron::new(router).http(bind).unwrap();
}
