pub struct IdStorage<T> {
    values: Vec<Option<T>>,
    free: Vec<usize>,
}

impl<T> IdStorage<T> {
    pub fn new() -> Self {
        Self { values: vec![], free: vec![] }
    }

    pub fn add(&mut self, value: T) -> usize {
        if let Some(id) = self.free.pop() {
            self.values[id] = Some(value);
            return id;
        }
        self.values.push(Some(value));
        self.values.len() - 1
    }

    pub fn set(&mut self, id: usize, value: T) {
        *self.values.get_mut(id)
            .expect("Id out of range")
            .as_mut()
            .expect("Id does not exist in storage; is the wrong type of id being used?") = value
    }

    pub fn remove(&mut self, id: usize) {
        *self.values.get_mut(id).expect("Id does not exist in storage; is the wrong type of id being used?") = None;
    }

    pub fn get(&self, id: usize) -> Option<&T> {
        self.values.get(id).expect("Id out of range").as_ref()
    }

    pub fn get_mut(&self, id: usize) -> Option<&mut T> {
        self.values.get(id).expect("Id out of range").as_mut()
    }
}

#[test]
fn test_id_storage() {
    let mut id_storage: IdStorage<Option<i32>> = IdStorage::new();
        
    let i1 = id_storage.add(Some(40));
    let i2 = id_storage.add(Some(32));
    let i3 = id_storage.add(None);
    assert_eq!(Some(40), *id_storage.get(i1).unwrap());
    assert_eq!(Some(32), *id_storage.get(i2).unwrap());
    assert_eq!(None, *id_storage.get(i3).unwrap());

    id_storage.remove(1);
    assert_eq!(Some(40), *id_storage.get(i1).unwrap());
    assert_eq!(None, id_storage.get(i3));

    id_storage.set(i3, Some(50));
    assert_eq!(Some(50), *id_storage.get(i3).unwrap());
}