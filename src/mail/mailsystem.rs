pub enum MailServices {
    TextMail, // UTF-8 Encoded Text
    MdMail, // Markdown-Mail
    Auth, // Authentication-Mail
    Image,
}

// Networking

// Mail is stored on Pivot Cluster


pub struct MailSystem {
    admin: String,
    moderators: Vec<String>,
}

pub struct MailAdminstrator {
    
}