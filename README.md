# temp_mail

### Async Rust wrapper of [1secmail](https://www.1secmail.com/api)

```rust
use temp_mail::TempMail;

let mut email = TempMail::new();
email.generate_email().await?;


// or

let mut email2 = TempMail::from_string("myemail@something.com").await?;

```

than you can retrieve:

## Email address

```rust
println!("{}", email.get_email());
```

## Email messages

```rust
email.check_inbox().await?;

let messages = email.get_messages();

```

## Open message

```rust
let message = email.get_message_by_id(id).await?

// message.id
// message.from
// message.subject
// message.date
// message.body
// message.text_body
// message.html_body
// message.attachments ->
// attachment.filename
// attachment.content_type
// attachment.size

```

## Download attachment

```rust
email.download_attachment(message.id, message.attachments[0].filename.clone(),"file.extension".into()).await?; // not working for images for now
```
