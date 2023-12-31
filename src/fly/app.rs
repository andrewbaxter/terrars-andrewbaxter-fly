use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderFly;

#[derive(Serialize)]
struct AppData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assign_shared_ip_address: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    org: Option<PrimField<String>>,
}

struct App_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AppData>,
}

#[derive(Clone)]
pub struct App(Rc<App_>);

impl App {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderFly) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `assign_shared_ip_address`.\nAssign a shared ipv4 address to the app. Note that depending on conditions an app may get a shared ip automatically."]
    pub fn set_assign_shared_ip_address(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().assign_shared_ip_address = Some(v.into());
        self
    }

    #[doc= "Set the field `org`.\nThe name of the organization to generate the app in, ex: `personal` (your initial org)"]
    pub fn set_org(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().org = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `app_url` after provisioning.\n"]
    pub fn app_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `assign_shared_ip_address` after provisioning.\nAssign a shared ipv4 address to the app. Note that depending on conditions an app may get a shared ip automatically."]
    pub fn assign_shared_ip_address(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.assign_shared_ip_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of application"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `org` after provisioning.\nThe name of the organization to generate the app in, ex: `personal` (your initial org)"]
    pub fn org(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.org", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `org_id` after provisioning.\n"]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.org_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shared_ip_address` after provisioning.\nA shared ipv4 address, automatically attached in certain conditions or if explicitly requested"]
    pub fn shared_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shared_ip_address", self.extract_ref()))
    }
}

impl Referable for App {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for App { }

impl ToListMappable for App {
    type O = ListRef<AppRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for App_ {
    fn extract_resource_type(&self) -> String {
        "fly_app".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApp {
    pub tf_id: String,
    #[doc= "Name of application"]
    pub name: PrimField<String>,
}

impl BuildApp {
    pub fn build(self, stack: &mut Stack) -> App {
        let out = App(Rc::new(App_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AppData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                assign_shared_ip_address: core::default::Default::default(),
                name: self.name,
                org: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AppRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `app_url` after provisioning.\n"]
    pub fn app_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `assign_shared_ip_address` after provisioning.\nAssign a shared ipv4 address to the app. Note that depending on conditions an app may get a shared ip automatically."]
    pub fn assign_shared_ip_address(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.assign_shared_ip_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of application"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `org` after provisioning.\nThe name of the organization to generate the app in, ex: `personal` (your initial org)"]
    pub fn org(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.org", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `org_id` after provisioning.\n"]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.org_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shared_ip_address` after provisioning.\nA shared ipv4 address, automatically attached in certain conditions or if explicitly requested"]
    pub fn shared_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shared_ip_address", self.extract_ref()))
    }
}
