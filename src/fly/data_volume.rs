use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderFly;

#[derive(Serialize)]
struct DataVolumeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    app: PrimField<String>,
    id: PrimField<String>,
}

struct DataVolume_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataVolumeData>,
}

#[derive(Clone)]
pub struct DataVolume(Rc<DataVolume_>);

impl DataVolume {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderFly) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Get a reference to the value of field `app` after provisioning.\nThe App this resource will be created in"]
    pub fn app(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nA fly-generated ID"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nA user-provided identifier, matching regexp `^[a-z0-9-]+$`"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nFly region, ex `ord`, `sin`, `mad`"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\nSize of volume in GB"]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }
}

impl Referable for DataVolume {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataVolume { }

impl ToListMappable for DataVolume {
    type O = ListRef<DataVolumeRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataVolume_ {
    fn extract_datasource_type(&self) -> String {
        "fly_volume".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataVolume {
    pub tf_id: String,
    #[doc= "The App this resource will be created in"]
    pub app: PrimField<String>,
    #[doc= "A fly-generated ID"]
    pub id: PrimField<String>,
}

impl BuildDataVolume {
    pub fn build(self, stack: &mut Stack) -> DataVolume {
        let out = DataVolume(Rc::new(DataVolume_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataVolumeData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                app: self.app,
                id: self.id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataVolumeRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVolumeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataVolumeRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `app` after provisioning.\nThe App this resource will be created in"]
    pub fn app(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nA fly-generated ID"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nA user-provided identifier, matching regexp `^[a-z0-9-]+$`"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nFly region, ex `ord`, `sin`, `mad`"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\nSize of volume in GB"]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }
}
