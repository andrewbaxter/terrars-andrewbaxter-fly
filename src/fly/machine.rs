use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderFly;

#[derive(Serialize)]
struct MachineData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    app: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cmd: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpus: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entrypoint: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exec: Option<ListField<PrimField<String>>>,
    image: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mounts: Option<Vec<MachineMountsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    region: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    services: Option<Vec<MachineServicesEl>>,
}

struct Machine_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<MachineData>,
}

#[derive(Clone)]
pub struct Machine(Rc<Machine_>);

impl Machine {
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

    #[doc= "Set the field `auto_destroy`.\nOptional boolean telling the Machine to destroy itself once it's complete"]
    pub fn set_auto_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_destroy = Some(v.into());
        self
    }

    #[doc= "Set the field `cmd`.\n"]
    pub fn set_cmd(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().cmd = Some(v.into());
        self
    }

    #[doc= "Set the field `cpu_type`.\nWhich machine flavor, ex: `shared`"]
    pub fn set_cpu_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cpu_type = Some(v.into());
        self
    }

    #[doc= "Set the field `cpus`.\n"]
    pub fn set_cpus(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().cpus = Some(v.into());
        self
    }

    #[doc= "Set the field `entrypoint`.\n"]
    pub fn set_entrypoint(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().entrypoint = Some(v.into());
        self
    }

    #[doc= "Set the field `env`.\nKeys and values must be strings"]
    pub fn set_env(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().env = Some(v.into());
        self
    }

    #[doc= "Set the field `exec`.\n"]
    pub fn set_exec(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().exec = Some(v.into());
        self
    }

    #[doc= "Set the field `memory`.\nAmount of memory in MB. `256`, `512`, `1024`, ..."]
    pub fn set_memory(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().memory = Some(v.into());
        self
    }

    #[doc= "Set the field `mounts`.\n"]
    pub fn set_mounts(self, v: impl Into<Vec<MachineMountsEl>>) -> Self {
        self.0.data.borrow_mut().mounts = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\nA user-provided identifier, matching regexp `^[a-z0-9-]+$`"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc= "Set the field `services`.\n"]
    pub fn set_services(self, v: impl Into<Vec<MachineServicesEl>>) -> Self {
        self.0.data.borrow_mut().services = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `app` after provisioning.\nThe App this resource will be created in"]
    pub fn app(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_destroy` after provisioning.\nOptional boolean telling the Machine to destroy itself once it's complete"]
    pub fn auto_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_destroy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cmd` after provisioning.\n"]
    pub fn cmd(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.cmd", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cpu_type` after provisioning.\nWhich machine flavor, ex: `shared`"]
    pub fn cpu_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cpus` after provisioning.\n"]
    pub fn cpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpus", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `entrypoint` after provisioning.\n"]
    pub fn entrypoint(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.entrypoint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `env` after provisioning.\nKeys and values must be strings"]
    pub fn env(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.env", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `exec` after provisioning.\n"]
    pub fn exec(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.exec", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nA fly-generated ID"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `image` after provisioning.\nProtocol-less docker image, ex: `registry.fly.io/myapp:mytag`"]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `memory` after provisioning.\nAmount of memory in MB. `256`, `512`, `1024`, ..."]
    pub fn memory(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mounts` after provisioning.\n"]
    pub fn mounts(&self) -> ListRef<MachineMountsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.mounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nA user-provided identifier, matching regexp `^[a-z0-9-]+$`"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_ip` after provisioning.\n"]
    pub fn private_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_ip", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nFly region, ex `ord`, `sin`, `mad`"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `services` after provisioning.\n"]
    pub fn services(&self) -> ListRef<MachineServicesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.services", self.extract_ref()))
    }
}

impl Referable for Machine {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Machine { }

impl ToListMappable for Machine {
    type O = ListRef<MachineRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Machine_ {
    fn extract_resource_type(&self) -> String {
        "fly_machine".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMachine {
    pub tf_id: String,
    #[doc= "The App this resource will be created in"]
    pub app: PrimField<String>,
    #[doc= "Protocol-less docker image, ex: `registry.fly.io/myapp:mytag`"]
    pub image: PrimField<String>,
    #[doc= "Fly region, ex `ord`, `sin`, `mad`"]
    pub region: PrimField<String>,
}

impl BuildMachine {
    pub fn build(self, stack: &mut Stack) -> Machine {
        let out = Machine(Rc::new(Machine_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(MachineData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
    shared: StackShared,
    base: String,
}

impl Ref for MachineRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MachineRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `app` after provisioning.\nThe App this resource will be created in"]
    pub fn app(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_destroy` after provisioning.\nOptional boolean telling the Machine to destroy itself once it's complete"]
    pub fn auto_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_destroy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cmd` after provisioning.\n"]
    pub fn cmd(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.cmd", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cpu_type` after provisioning.\nWhich machine flavor, ex: `shared`"]
    pub fn cpu_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cpus` after provisioning.\n"]
    pub fn cpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpus", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `entrypoint` after provisioning.\n"]
    pub fn entrypoint(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.entrypoint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `env` after provisioning.\nKeys and values must be strings"]
    pub fn env(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.env", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `exec` after provisioning.\n"]
    pub fn exec(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.exec", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nA fly-generated ID"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `image` after provisioning.\nProtocol-less docker image, ex: `registry.fly.io/myapp:mytag`"]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `memory` after provisioning.\nAmount of memory in MB. `256`, `512`, `1024`, ..."]
    pub fn memory(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mounts` after provisioning.\n"]
    pub fn mounts(&self) -> ListRef<MachineMountsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.mounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nA user-provided identifier, matching regexp `^[a-z0-9-]+$`"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_ip` after provisioning.\n"]
    pub fn private_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_ip", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nFly region, ex `ord`, `sin`, `mad`"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `services` after provisioning.\n"]
    pub fn services(&self) -> ListRef<MachineServicesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.services", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct MachineMountsEl {
    path: PrimField<String>,
    volume: PrimField<String>,
}

impl MachineMountsEl { }

impl ToListMappable for MachineMountsEl {
    type O = BlockAssignable<MachineMountsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMachineMountsEl {
    #[doc= "Path for volume to be mounted on vm, ex: `/data`"]
    pub path: PrimField<String>,
    #[doc= "ID of volume"]
    pub volume: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for MachineMountsElRef {
    fn new(shared: StackShared, base: String) -> MachineMountsElRef {
        MachineMountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MachineMountsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\nPath for volume to be mounted on vm, ex: `/data`"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc= "Get a reference to the value of field `volume` after provisioning.\nID of volume"]
    pub fn volume(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume", self.base))
    }
}

#[derive(Serialize)]
pub struct MachineServicesElPortsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    force_https: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    handlers: Option<ListField<PrimField<String>>>,
    port: PrimField<f64>,
}

impl MachineServicesElPortsEl {
    #[doc= "Set the field `force_https`.\nAutomatically redirect to HTTPS on \"http\" handler"]
    pub fn set_force_https(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.force_https = Some(v.into());
        self
    }

    #[doc= "Set the field `handlers`.\nHow the edge should process requests; ex empty, or `tls` to attach app's certificate"]
    pub fn set_handlers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.handlers = Some(v.into());
        self
    }
}

impl ToListMappable for MachineServicesElPortsEl {
    type O = BlockAssignable<MachineServicesElPortsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMachineServicesElPortsEl {
    #[doc= "Mapped external port number"]
    pub port: PrimField<f64>,
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
    shared: StackShared,
    base: String,
}

impl Ref for MachineServicesElPortsElRef {
    fn new(shared: StackShared, base: String) -> MachineServicesElPortsElRef {
        MachineServicesElPortsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MachineServicesElPortsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `force_https` after provisioning.\nAutomatically redirect to HTTPS on \"http\" handler"]
    pub fn force_https(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.force_https", self.base))
    }

    #[doc= "Get a reference to the value of field `handlers` after provisioning.\nHow the edge should process requests; ex empty, or `tls` to attach app's certificate"]
    pub fn handlers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.handlers", self.base))
    }

    #[doc= "Get a reference to the value of field `port` after provisioning.\nMapped external port number"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct MachineServicesEl {
    internal_port: PrimField<f64>,
    ports: Vec<MachineServicesElPortsEl>,
    protocol: PrimField<String>,
}

impl MachineServicesEl { }

impl ToListMappable for MachineServicesEl {
    type O = BlockAssignable<MachineServicesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMachineServicesEl {
    #[doc= "Port the machine listens on"]
    pub internal_port: PrimField<f64>,
    #[doc= "How the port is exposed"]
    pub ports: Vec<MachineServicesElPortsEl>,
    #[doc= "`udp` or `tcp`"]
    pub protocol: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for MachineServicesElRef {
    fn new(shared: StackShared, base: String) -> MachineServicesElRef {
        MachineServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MachineServicesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `internal_port` after provisioning.\nPort the machine listens on"]
    pub fn internal_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.internal_port", self.base))
    }

    #[doc= "Get a reference to the value of field `ports` after provisioning.\nHow the port is exposed"]
    pub fn ports(&self) -> ListRef<MachineServicesElPortsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }

    #[doc= "Get a reference to the value of field `protocol` after provisioning.\n`udp` or `tcp`"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
}
