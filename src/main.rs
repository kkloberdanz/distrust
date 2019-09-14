// This file is part of Distrust.
//
// Distrust is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Distrust is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with Distrust.  If not, see <https://www.gnu.org/licenses/>.
//
// Programmer: Kyle Kloberdanz
// Date: 09 Sep 2019

use actix_files;
use actix_web::{
    web, App, HttpRequest, HttpResponse, Responder, error, Error, HttpServer
};
use std::path::PathBuf;
use std::cell::Cell;
use std::fs;
use std::io::Write;

use actix_multipart::{Field, Multipart, MultipartError};
use futures::future::{err, Either};
use futures::{Future, Stream};

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

fn home() -> impl Responder {
    HttpResponse::Ok().body("Welcome")
}

fn redirect() -> impl Responder {
    HttpResponse::TemporaryRedirect()
        .header("Location", "https://www.youtube.com")
        .finish()
}

fn get_file(req: HttpRequest) -> actix_web::Result<actix_files::NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    println!("serving file: {:?}", path);
    Ok(actix_files::NamedFile::open(path)?)
}

    
//fn put_file(req: HttpRequest) -> actix_web::Result<String> {
fn put_file(req: HttpRequest, payload: web::Payload) -> actix_web::Result<String> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    println!("saving to file: {:?}", path);
    println!("req = {:#?}", req);
    let file = fs::File::create(path)?;

    //let response = format!("uploaded");
    let data = payload.concat2();
    /*
    data
        .fold(web::BytesMut::new(), move |mut body, chunk| {
            body.extend_from_slice(&chunk);
            Ok::<_, Error>(body)
        })
        .and_then(|body| {
            format!("Body {:?}!", body);
            Ok(HttpResponse::Ok().finish())
        });
    //println!("data = {:?}", data);
    */
    Ok("uploaded".to_string()) // HttpResponse::Ok().body(response)
}

fn test(req: HttpRequest, body: web::Payload) -> impl Future<Item = HttpResponse, Error = Error> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    println!("saving to file: {:?}", path);
    println!("req = {:#?}", req);
    body.map_err(Error::from)
        .fold(web::BytesMut::new(), move |mut body, chunk| {
            body.extend_from_slice(&chunk);
            Ok::<_, Error>(body)
        })
        .and_then(|body| {
            format!("Body {:?}!", body);
            println!("body = {:?}", body);
            let mut file = fs::File::create(path).unwrap();
            file.write_all(&body);
            Ok(HttpResponse::Ok().finish())
        })
}

fn main() {
    println!("Hello, world!");
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/greet")
                    .route("/", web::get().to(index))
                    .route("/again", web::get().to(index2))
                    .route("/redirect", web::get().to(redirect)),
            )
            .service(
                actix_files::Files::new("/static", ".").show_files_listing(),
            )
            .service(
                web::scope("/get")
                    .route("/{filename:.*}", web::get().to(get_file)),
            )
            .service(
                web::scope("/put")
                    .route("/{filename:.*}", web::put().to(put_file))
                    //.route("/{filename:.*}", web::post().to_async(upload)),
            )
            .service(
                web::scope("/test")
                    .route("/{filename:.*}", web::put().to_async(test))
            )
            .route("/", web::get().to(home))
    })
    .workers(16 * 4)
    .bind("0.0.0.0:8086")
    .unwrap()
    .run()
    .unwrap();
}
