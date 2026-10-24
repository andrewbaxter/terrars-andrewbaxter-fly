use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderFly;

#[derive(Serialize)]
struct MachineData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    app: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_destroy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cmd: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpus: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entrypoint: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exec: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    image: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mounts: std::option::Option<std::vec::Vec<MachineMountsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    region: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    services: std::option::Option<std::vec::Vec<MachineServicesEl>>,
}

struct Machine_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MachineData>,
}

#[derive(Clone)]
pub struct Machine(std::rc::Rc<Machine_>);

impl Machine {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderFly) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
        self
    }

    pub fn add_lifecycle_precondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.precondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    pub fn add_lifecycle_postcondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.postcondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    #[doc = "Set the field `auto_destroy`.\nOptional boolean telling the Machine to destroy itself once it's complete"]
    pub fn set_auto_destroy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_destroy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cmd`.\n"]
    pub fn set_cmd(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().cmd = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_type`.\nWhich machine flavor, ex: `shared`"]
    pub fn set_cpu_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cpu_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpus`.\n"]
    pub fn set_cpus(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().cpus = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `entrypoint`.\n"]
    pub fn set_entrypoint(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().entrypoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `env`.\nKeys and values must be strings"]
    pub fn set_env(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().env = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exec`.\n"]
    pub fn set_exec(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().exec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory`.\nAmount of memory in MB. `256`, `512`, `1024`, ..."]
    pub fn set_memory(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().memory = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mounts`.\n"]
    pub fn set_mounts(self, v: impl std::convert::Into<std::vec::Vec<MachineMountsEl>>) -> Self {
        self.0.data.borrow_mut().mounts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nA user-provided identifier, matching regexp `^[a-z0-9-]+$`"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `services`.\n"]
    pub fn set_services(self, v: impl std::convert::Into<std::vec::Vec<MachineServicesEl>>) -> Self {
        self.0.data.borrow_mut().services = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `app` after provisioning.\nThe App this resource will be created in"]
    pub fn app(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auto_destroy` after provisioning.\nOptional boolean telling the Machine to destroy itself once it's complete"]
    pub fn auto_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cmd` after provisioning.\n"]
    pub fn cmd(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cmd", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cpu_type` after provisioning.\nWhich machine flavor, ex: `shared`"]
    pub fn cpu_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cpus` after provisioning.\n"]
    pub fn cpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpus", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entrypoint` after provisioning.\n"]
    pub fn entrypoint(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entrypoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `env` after provisioning.\nKeys and values must be strings"]
    pub fn env(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.env", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exec` after provisioning.\n"]
    pub fn exec(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nA fly-generated ID"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `image` after provisioning.\nProtocol-less docker image, ex: `registry.fly.io/myapp:mytag`"]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memory` after provisioning.\nAmount of memory in MB. `256`, `512`, `1024`, ..."]
    pub fn memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mounts` after provisioning.\n"]
    pub fn mounts(&self) -> terrars::ListRef<MachineMountsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA user-provided identifier, matching regexp `^[a-z0-9-]+$`"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_ip` after provisioning.\n"]
    pub fn private_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nFly region, ex `ord`, `sin`, `mad`"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `services` after provisioning.\n"]
    pub fn services(&self) -> terrars::ListRef<MachineServicesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.services", self.extract_ref()))
    }
}

impl terrars::Referable for Machine {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Machine { }

impl terrars::ToListMappable for Machine {
    type O = terrars::ListRef<MachineRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Machine_ {
    fn extract_resource_type(&self) -> std::string::String {
        "fly_machine".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMachine {
    pub tf_id: std::string::String,
    #[doc = "The App this resource will be created in"]
    pub app: terrars::PrimField<std::string::String>,
    #[doc = "Protocol-less docker image, ex: `registry.fly.io/myapp:mytag`"]
    pub image: terrars::PrimField<std::string::String>,
    #[doc = "Fly region, ex `ord`, `sin`, `mad`"]
    pub region: terrars::PrimField<std::string::String>,
}

impl BuildMachine {
    pub fn build(self, stack: &mut terrars::Stack) -> Machine {
        let out = Machine(std::rc::Rc::new(Machine_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MachineData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                app: self.app,
                auto_destroy: core::default::Default::default(),
                cmd: core::default::Default::default(),
                cpu_type: core::default::Default::default(),
                cpus: core::default::Default::default(),
                entrypoint: core::default::Default::default(),
                env: core::default::Default::default(),
                exec: core::default::Default::default(),
                image: self.image,
                memory: core::default::Default::default(),
                mounts: core::default::Default::default(),
                name: core::default::Default::default(),
                region: self.region,
                services: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MachineRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MachineRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MachineRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app` after provisioning.\nThe App this resource will be created in"]
    pub fn app(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auto_destroy` after provisioning.\nOptional boolean telling the Machine to destroy itself once it's complete"]
    pub fn auto_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cmd` after provisioning.\n"]
    pub fn cmd(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cmd", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cpu_type` after provisioning.\nWhich machine flavor, ex: `shared`"]
    pub fn cpu_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cpus` after provisioning.\n"]
    pub fn cpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpus", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entrypoint` after provisioning.\n"]
    pub fn entrypoint(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entrypoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `env` after provisioning.\nKeys and values must be strings"]
    pub fn env(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.env", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exec` after provisioning.\n"]
    pub fn exec(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nA fly-generated ID"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `image` after provisioning.\nProtocol-less docker image, ex: `registry.fly.io/myapp:mytag`"]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memory` after provisioning.\nAmount of memory in MB. `256`, `512`, `1024`, ..."]
    pub fn memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mounts` after provisioning.\n"]
    pub fn mounts(&self) -> terrars::ListRef<MachineMountsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA user-provided identifier, matching regexp `^[a-z0-9-]+$`"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_ip` after provisioning.\n"]
    pub fn private_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nFly region, ex `ord`, `sin`, `mad`"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `services` after provisioning.\n"]
    pub fn services(&self) -> terrars::ListRef<MachineServicesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.services", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct MachineMountsEl {
    path: terrars::PrimField<std::string::String>,
    volume: terrars::PrimField<std::string::String>,
}

impl MachineMountsEl { }

impl terrars::ToListMappable for MachineMountsEl {
    type O = terrars::BlockAssignable<MachineMountsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMachineMountsEl {
    #[doc = "Path for volume to be mounted on vm, ex: `/data`"]
    pub path: terrars::PrimField<std::string::String>,
    #[doc = "ID of volume"]
    pub volume: terrars::PrimField<std::string::String>,
}

impl BuildMachineMountsEl {
    pub fn build(self) -> MachineMountsEl {
        MachineMountsEl {
            path: self.path,
            volume: self.volume,
        }
    }
}

pub struct MachineMountsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MachineMountsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MachineMountsElRef {
        MachineMountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MachineMountsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nPath for volume to be mounted on vm, ex: `/data`"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `volume` after provisioning.\nID of volume"]
    pub fn volume(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume", self.base))
    }
}

#[derive(Serialize)]
pub struct MachineServicesElPortsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_https: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    handlers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    port: terrars::PrimField<f64>,
}

impl MachineServicesElPortsEl {
    #[doc = "Set the field `force_https`.\nAutomatically redirect to HTTPS on \"http\" handler"]
    pub fn set_force_https(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.force_https = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `handlers`.\nHow the edge should process requests; ex empty, or `tls` to attach app's certificate"]
    pub fn set_handlers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.handlers = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MachineServicesElPortsEl {
    type O = terrars::BlockAssignable<MachineServicesElPortsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMachineServicesElPortsEl {
    #[doc = "Mapped external port number"]
    pub port: terrars::PrimField<f64>,
}

impl BuildMachineServicesElPortsEl {
    pub fn build(self) -> MachineServicesElPortsEl {
        MachineServicesElPortsEl {
            force_https: core::default::Default::default(),
            handlers: core::default::Default::default(),
            port: self.port,
        }
    }
}

pub struct MachineServicesElPortsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MachineServicesElPortsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MachineServicesElPortsElRef {
        MachineServicesElPortsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MachineServicesElPortsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `force_https` after provisioning.\nAutomatically redirect to HTTPS on \"http\" handler"]
    pub fn force_https(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_https", self.base))
    }

    #[doc =
        "Get a reference to the value of field `handlers` after provisioning.\nHow the edge should process requests; ex empty, or `tls` to attach app's certificate"]
    pub fn handlers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.handlers", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\nMapped external port number"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct MachineServicesEl {
    internal_port: terrars::PrimField<f64>,
    ports: std::vec::Vec<MachineServicesElPortsEl>,
    protocol: terrars::PrimField<std::string::String>,
}

impl MachineServicesEl { }

impl terrars::ToListMappable for MachineServicesEl {
    type O = terrars::BlockAssignable<MachineServicesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMachineServicesEl {
    #[doc = "Port the machine listens on"]
    pub internal_port: terrars::PrimField<f64>,
    #[doc = "How the port is exposed"]
    pub ports: std::vec::Vec<MachineServicesElPortsEl>,
    #[doc = "`udp` or `tcp`"]
    pub protocol: terrars::PrimField<std::string::String>,
}

impl BuildMachineServicesEl {
    pub fn build(self) -> MachineServicesEl {
        MachineServicesEl {
            internal_port: self.internal_port,
            ports: self.ports,
            protocol: self.protocol,
        }
    }
}

pub struct MachineServicesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MachineServicesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MachineServicesElRef {
        MachineServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MachineServicesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `internal_port` after provisioning.\nPort the machine listens on"]
    pub fn internal_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.internal_port", self.base))
    }

    #[doc = "Get a reference to the value of field `ports` after provisioning.\nHow the port is exposed"]
    pub fn ports(&self) -> terrars::ListRef<MachineServicesElPortsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n`udp` or `tcp`"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
}
