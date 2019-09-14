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
    web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use futures::{Future, Stream};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

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

fn put_file(
    req: HttpRequest,
    body: web::Payload,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    println!("saving to file: {:?}", path);
    println!("req = {:#?}", req);
    body.map_err(Error::from)
        .fold(web::BytesMut::new(), move |mut body, chunk| {
            body.extend_from_slice(&chunk);
            Ok::<_, Error>(body)
        })
        .and_then(|body| {
            let mut file =
                fs::File::create(path).expect("failed to open file");
            file.write_all(&body).expect("failed to write");
            Ok(HttpResponse::Ok().finish())
        })
}

fn main() {
    println!("Hello, world!");
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/greet")
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
                    .route("/{filename:.*}", web::put().to_async(put_file)),
            )
            .route("/", web::get().to(home))
    })
    .workers(16 * 4)
    .bind("0.0.0.0:8086")
    .expect("failed to bind to address and port")
    .run()
    .expect("failed to run web server");
}
