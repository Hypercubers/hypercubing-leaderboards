use crate::env;
use crate::error::AppResult;

pub async fn send_email(
    recipient: &str,
    subject: &str,
    text_body: &str,
    html_body: &str,
) -> AppResult {
    let message = mail_send::mail_builder::MessageBuilder::new()
        .from((&**env::SMTP_FROM_NAME, &**env::SMTP_FROM_ADDRESS))
        .to(recipient)
        .subject(subject)
        .text_body(text_body)
        .html_body(html_body);

    mail_send::SmtpClientBuilder::new(&**env::SMTP_HOST, *env::SMTP_HOST_PORT)
        .credentials((&**env::SMTP_USERNAME, &**env::SMTP_PASSWORD))
        .connect()
        .await?
        .send(message)
        .await?;

    tracing::debug!("Sending email to {recipient}");

    Ok(())
}
