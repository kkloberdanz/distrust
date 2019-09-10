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
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

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

fn main() {
    println!("Hello, world!");
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/greet")
                    .route("/", web::get().to(index))
                    .route("/again", web::get().to(index2)),
            )
            .service(
                actix_files::Files::new("/static", ".").show_files_listing(),
            )
            .route("/", web::get().to(home))
    })
    .workers(16 * 4)
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
