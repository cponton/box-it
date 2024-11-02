use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct BoxRequest {
    number_of_pieces: u32,
    length: f32,
    width: f32,
    height: f32,
    weight: f32,
}

#[post("/calculate_boxes")]
async fn calculate_boxes(box_request: web::Json<BoxRequest>) -> impl Responder {
    // Constants for the larger box dimensions and max weight
    const BOX_VOLUME: f32 = 20.0 * 20.0 * 20.0;
    const MAX_WEIGHT: f32 = 40.0;

    // Calculate the volume and weight for one piece
    let piece_volume = box_request.length * box_request.width * box_request.height;
    let piece_weight = box_request.weight;

    // Determine the total volume and weight needed
    let total_volume = piece_volume * box_request.number_of_pieces as f32;
    let total_weight = piece_weight * box_request.number_of_pieces as f32;

    // Calculate the number of boxes needed based on volume and weight constraints
    let boxes_by_volume = (total_volume / BOX_VOLUME).ceil() as u32;
    let boxes_by_weight = (total_weight / MAX_WEIGHT).ceil() as u32;

    // Choose the larger number of boxes needed to satisfy both constraints
    let boxes_needed = boxes_by_volume.max(boxes_by_weight);

    HttpResponse::Ok().json(serde_json::json!({ "boxes_needed": boxes_needed }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(calculate_boxes)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
