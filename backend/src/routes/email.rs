use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};


pub async fn send_email(destination: String, subject: &str, content: &str) -> Result<(), lettre::error::Error>{

    //build email
    let email = Message::builder()
    .to(destination.parse().unwrap())
    .from("admin@brusselsstudentguide.be".parse().unwrap())
    .subject(subject)
    .body(content.to_string())?;

    //smtp credentials (still need to be defined)
    let credentials = Credentials::new("username".to_owned(),"password".to_owned());
    
    //Send mail through smtp server (still need to define smtp server)
    let mailer = SmtpTransport::relay("smtp.brusselsstudentguide.be")
        .unwrap()
        .credentials(credentials)
        .build();

    mailer.send(&email);
    Ok(())
}