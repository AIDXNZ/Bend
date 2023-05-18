use crdts::VClock;

#[derive(Clone)]
pub struct AuthenticatedMatrixClock {
    pub(crate) root_key: Option<Vec<u8>>,
    pub local_casual: VClock<String>,
    pub global_: Vec<VClock<String>>,
    last_event: Option<String>,
    size: Option<i32>,
}

impl AuthenticatedMatrixClock {
    pub fn new() -> Self {
        Self { root_key: None, local_casual: VClock::new(), global_: vec![], last_event: None, size: Some(3) }
    }

    fn concurent(&self) -> bool {
        let idk =  self.global_.first().unwrap().get(&"A".to_string());
        let local = self.local_casual.get(&"A".to_string());
        if idk == local{
            return false;
        } else {
            return true;
        }
    }

    
}