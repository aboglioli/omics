use std::io::Write;

use actix_multipart::Multipart;
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use bytes::Bytes;
use futures::StreamExt;

use common::request::CommandResponse;

#[derive(Debug, Clone)]
pub struct File {
    name: String,
    path: String,
}

impl File {
    pub fn new<S: Into<String>>(name: S) -> Self {
        let name = name.into();

        File {
            name: name.clone(),
            path: format!("./tmp/{}", name),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

// POST /upload
async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut files: Vec<File> = Vec::new();
    let mut data = Bytes::new();

    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_type = field.content_disposition().unwrap();
        let name = content_type.get_name().unwrap();

        if name == "data" {
            while let Some(chunk) = field.next().await {
                data = chunk?;
            }
        } else {
            match content_type.get_filename() {
                Some(filename) => {
                    let file = File::new(sanitize_filename::sanitize(&filename));
                    let path = file.path().to_owned();
                    let mut f = web::block(move || std::fs::File::create(path)).await?;

                    while let Some(chunk) = field.next().await {
                        let data = chunk?;
                        f = web::block(move || f.write_all(&data).map(|_| f)).await?;
                    }

                    files.push(file.clone());
                }
                None => {
                    println!("No file");
                }
            }
        }
    }

    println!("bytes = {:#?}", data);
    println!("files = {:#?}", files);

    Ok(HttpResponse::Ok().json(CommandResponse::default()))
}

// GET /upload
fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/" method="post" enctype="multipart/form-data" id="myForm" >
                <input type="text"  id="text" name="text" value="test_text"/>
                <input type="number"  id="number" name="number" value="123123"/>

                <input type="button" value="Submit" onclick="myFunction()"></button>
            </form>
            <input type="file" multiple name="file" id="myFile"/>
        </body>
        <script>
        function myFunction(){
            var myForm = document.getElementById('myForm');
            var myFile = document.getElementById('myFile');

            let formData = new FormData();
            const obj = {
                text: document.getElementById('text').value,
                number: Number(document.getElementById('number').value)
            };
            const json = JSON.stringify(obj);
            console.log(obj);
            console.log(json);


            formData.append("data", json);
            formData.append("myFile", myFile.files[0]);

            var request = new XMLHttpRequest();
            request.open("POST", "");
            request.send(formData);
        }


        </script>
    </html>"#;

    HttpResponse::Ok().body(html)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    std::fs::create_dir_all("./tmp").unwrap();

    cfg.service(
        web::scope("/upload")
            .route("", web::get().to(index))
            .route("", web::post().to(upload)),
    );
}
