#![allow(non_camel_case_types)]

use serde::Serialize;

#[derive(Serialize)]
struct ProviderFlyData_ {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alias: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fly_api_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fly_http_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct ProviderFly_ {
    data: std::cell::RefCell<ProviderFlyData_>,
}

pub struct ProviderFly(std::rc::Rc<ProviderFly_>);

impl ProviderFly {
    pub fn provider_ref(&self) -> std::string::String {
        let data = self.0.data.borrow();
        if let std::option::Option::Some(alias) = &data.alias {
            format!("{}.{}", "fly", alias)
        } else {
            "fly".into()
        }
    }

    pub fn set_alias(self, alias: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().alias = std::option::Option::Some(alias.to_string());
        self
    }

    #[doc = "Set the field `fly_api_token`.\nfly.io api token. If not set checks env for FLY_API_TOKEN"]
    pub fn set_fly_api_token(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().fly_api_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fly_http_endpoint`.\nWhere the provider should look to find the fly http endpoint"]
    pub fn set_fly_http_endpoint(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().fly_http_endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::Provider for ProviderFly_ {
    fn extract_type_tf_id(&self) -> std::string::String {
        "fly".into()
    }

    fn extract_provider_type(&self) -> serde_json::Value {
        serde_json::json!({
            "source": "andrewbaxter/fly",
            "version": "0.1.18",
        })
    }

    fn extract_provider(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProviderFly {}

impl BuildProviderFly {
    pub fn build(self, stack: &mut terrars::Stack) -> ProviderFly {
        let out = ProviderFly(std::rc::Rc::new(ProviderFly_ { data: std::cell::RefCell::new(ProviderFlyData_ {
            alias: std::option::Option::None,
            fly_api_token: core::default::Default::default(),
            fly_http_endpoint: core::default::Default::default(),
        }) }));
        stack.add_provider(out.0.clone());
        out
    }
}
