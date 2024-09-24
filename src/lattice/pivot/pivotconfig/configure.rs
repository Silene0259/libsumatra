use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct ConfigurationRules {
    mail_allow_incoming: bool,
    mail_allow_outgoing: bool,
    mail_security: bool,
    
    storage_allow_domains: bool,
    storage_allow_cert: bool,
    storage_allow_file: bool,
    
    slab_allow_users: bool,
}


/* Layout

pivot/strg/


*/
#[derive(Serialize,Deserialize)]
pub struct DomainSpaceConfig {
    allow_images: bool,
    allow_text: bool,
}

#[derive(Serialize,Deserialize)]
pub struct PivotConfig {
    id: u16,
    config: ConfigurationRules,
    domainspaceconfig: DomainSpaceConfig,
}