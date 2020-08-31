use futures::{Stream, StreamExt};
use actix_multipart::Multipart;
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};

use common::request::CommandResponse;

// POST /upload
async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Some(item) = payload.next().await {
        let mut field = item?;

        println!("{}", field.content_disposition().unwrap().get_name().unwrap());

        while let Some(chunk) = field.next().await {
            println!("-- CHUNK: \n{:?}", std::str::from_utf8(&chunk?));
        }
    }

    Ok(HttpResponse::Ok().into())
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
    cfg.service(
        web::scope("/upload")
            .route("", web::get().to(index))
            .route("", web::post().to(upload)),
    );
}
