//! istio model — service mesh.

#[derive(Debug, Clone)]
pub struct Service {
    pub name: String,
    pub namespace: String,
    pub hosts: Vec<String>,
    pub ports: Vec<u16>,
}

#[derive(Debug, Clone)]
pub struct VirtualService {
    pub name: String,
    pub hosts: Vec<String>,
    pub gateways: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DestinationRule {
    pub name: String,
    pub host: String,
    pub traffic_policy: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub services: Vec<Service>,
    pub virtual_services: Vec<VirtualService>,
    pub destination_rules: Vec<DestinationRule>,
}

impl Default for Model {
    fn default() -> Self { Self { services: vec![], virtual_services: vec![], destination_rules: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_service(&mut self, service: Service) { self.services.push(service); }
    pub fn add_virtual_service(&mut self, vs: VirtualService) { self.virtual_services.push(vs); }

    pub fn list_services(&self) -> &[Service] { &self.services }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_service() {
        let mut i = Model::new();
        i.add_service(Service { name: "web".into(), namespace: "default".into(), hosts: vec!["web.example.com".into()], ports: vec![80] });
        i.add_virtual_service(VirtualService { name: "web-vs".into(), hosts: vec!["web.example.com".into()], gateways: vec!["web-gateway".into()] });
        assert_eq!(i.list_services().len(), 1);
        assert_eq!(i.virtual_services.len(), 1);
    }
}
