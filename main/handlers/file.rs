use actix_multipart::Multipart;
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

use file::file::UploadedFile;
use file::uploader::{FileUploader, S3FileUploader};

use crate::error::PublicError;

#[derive(Serialize)]
pub struct UploadResponse {
    files: Vec<UploadedFile>,
}

// POST /upload
async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let (data, files) = file::extract_payload(&mut payload)
        .await
        .map_err(PublicError::from)?;

    println!("bytes = {:#?}", data);
    println!("files = {:#?}", files);

    let uploader = S3FileUploader::new();

    let mut uploaded_files = Vec::new();
    for file in files.into_iter() {
        let uploaded_file = uploader.upload(file).await.map_err(PublicError::from)?;
        uploaded_files.push(uploaded_file);
    }

    println!("uploaded_file = {:#?}", uploaded_files);

    Ok(HttpResponse::Ok().json(UploadResponse {
        files: uploaded_files,
    }))
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
