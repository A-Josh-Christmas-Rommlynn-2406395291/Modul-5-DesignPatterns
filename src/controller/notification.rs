use rocket::response::status::Created;
use rocket::serde::json::Json;

use crate::model::subscriber::Subscriber;
use crate::service::notification::NotificationService;
use bambangshop::Result;

#[post("/subscribe/<product_type>", data = "<subscriber>")]
pub fn subscribe(
    product_type: &str,
    subscriber: Json<Subscriber>,
) -> Result<Created<Json<Subscriber>>> {
    return match NotificationService::subscribe(product_type, subscriber.into_inner()) {
        Ok(f) => Ok(Created::new("/").body(Json::from(f))),
        Err(e) => Err(e),
    };
}

