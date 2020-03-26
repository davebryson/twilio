use std::fmt;

#[macro_use]
extern crate actix_web;

use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

// This should eventually handle the response we want to send back to the caller
fn generic_response(resp: String) -> String {
  format!(
    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
  <Response>
      <Message><Body>you said: {}</Body></Message>
  </Response>",
    resp
  )
}

// TODO only parse fields needed - remove the rest - keep simple
#[derive(Debug, Clone, Deserialize)]
pub struct WebhookRequest {
  #[serde(rename(deserialize = "MessageSid"))]
  pub message_sid: String,
  #[serde(rename(deserialize = "AccountSid"))]
  pub account_sid: String,
  #[serde(rename(deserialize = "From"))]
  pub from: String,
  #[serde(rename(deserialize = "To"))]
  pub to: String,
  #[serde(rename(deserialize = "Body"))]
  pub body: String,
  #[serde(rename(deserialize = "ApiVersion"))]
  pub api_version: String,
}

impl fmt::Display for WebhookRequest {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    writeln!(f, "from       : {}", self.from)?;
    writeln!(f, "to         : {}", self.to)?;
    writeln!(f, "body       : {}", self.body)?;
    writeln!(f, "message sid: {}", self.message_sid)?;
    writeln!(f, "account sid: {}", self.account_sid)
  }
}

/// Handler
#[post("/sms")]
async fn index(form: web::Form<WebhookRequest>) -> impl Responder {
  println!("{}", form.clone());
  // get the sender
  // get the body
  // act on the body
  //HttpResponse::Ok()
  //  .content_type("application/xml")
  // .body(generic_response(form.body.clone()))
  HttpResponse::Ok()
}
