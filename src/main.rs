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
    http, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use http::StatusCode;
use std::path::PathBuf;

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

fn not_found() -> impl Responder {
    HttpResponse::Ok().body("Zoinks! not found!")
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
            .route("/", web::get().to(home))
    })
    .workers(16 * 4)
    .bind("0.0.0.0:8086")
    .unwrap()
    .run()
    .unwrap();
}
