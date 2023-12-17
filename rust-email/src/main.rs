use lettre::{
    message::{header, MultiPart, SinglePart},
    message::header::ContentType, 
    transport::smtp::authentication::Credentials,
    Message, Transport, SmtpTransport,
};

fn main() {

    let email = Message::builder()
        .from("NoBody <nobody@gmail.com>".parse().unwrap())
        .reply_to("Yuin <noreply@gmail.com>".parse().unwrap())
        .to("JMR <johnmarkroco05@gmail.com>".parse().unwrap())
        .subject("Hello from Lettre!")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from("Be happy!"))
        .unwrap();

        //Create App Password in Gmail
        let creds = Credentials::new("username@gmail.com".to_owned(), "app-password".to_owned());

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {e:?}"),
        };

}
