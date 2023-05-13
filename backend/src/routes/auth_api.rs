use serde::{Deserialize, Serialize};
<<<<<<< Updated upstream
use actix_web:: {web, Result, HttpResponse};

=======
use actix_web:: {web};
>>>>>>> Stashed changes

#[derive(Debug, Deserialize, Serialize)]
struct AuthToken{
    token: String,
}

//middleware function
async fn middleware(
<<<<<<< Updated upstream
    request: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
) -> Result<actix_web::HttpResponse, actix_web::Error>{
    //Take header token
    let header_token = request.headers().get("Authorization");
    let token = match header_token{
        Some(token) => token.to_str().unwrap_or(""),
        None => "",
    };

    //Token validation
    if token_validation(token){
        	let response = HttpResponse::Ok().body("Private route accessed");
            Ok(response)
    }
    else{
        let response = HttpResponse::Unauthorized().json(json!({"error": "Invalid token"}));
        Ok(response)
    }
}

    fn token_validation(token: &str) -> bool{
        token == 000
    }


=======
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
){}
>>>>>>> Stashed changes

