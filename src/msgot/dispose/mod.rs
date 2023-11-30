mod util;

use super::msgot::{Link, BasicLink};

pub struct WholeConfig {
    pub domain: String
}

#[derive(Debug, Default)]
pub struct DomainRelevance {
    pub sub_domain: Vec<BasicLink>,
    pub relevance_domain: Vec<BasicLink>
}

pub fn dispose_focus(link: &Link, basic_config: &WholeConfig) {
    let subdomain = &link.subdomain;
    let domain = &basic_config.domain;

    let mut clean_domain = domain.trim().to_owned();

    let mut domain_list = DomainRelevance::default();

    for subdo in subdomain.iter() {
        if util::is_subdomain(&subdo.url) {
            let mut real_domain;
            if clean_domain.trim().ends_with("/") {
                clean_domain.pop();
                real_domain = clean_domain.trim();
            } else {
                real_domain = clean_domain.trim();
            }
            domain_list.sub_domain.push(BasicLink{
                text: subdo.text.clone(),
                url: format!("{}{}", real_domain.clone(), subdo.url.clone())
            });
        }
    }
    println!("domain_list:{:?}", domain_list);
}