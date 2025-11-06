use super::commons::SerializeDeserialize;

type ContainerType = Vec<Box<dyn SerializeDeserialize>>;
type FSPECType = Vec<u8>;

pub struct IFSPEC {
    profile: ContainerType,
    fspec: FSPECType
}

impl IFSPEC {
    pub fn calculate_fspec(&self) -> () {

    }

    pub fn add_item_to_profile(&mut self, item: impl SerializeDeserialize + 'static) -> () {
        self.profile.push(Box::new(item));
    }

    pub fn remove_item_from_profile(&mut self, index: usize) -> Option<Box<dyn SerializeDeserialize>> {
        if index < self.profile.len() {
            Some(self.profile.remove(index))
        } else {
            None
        }
    }
}