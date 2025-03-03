struct File {
    id: i32,
    parent: i32,
    range: (i32, i32)
}
impl File {
    pub fn init(id: i32, parent: i32, range: (i32, i32)) -> Self {
        Self {
            id: id,
            parent: parent,
            range: range
        }
    }
}

pub struct FileSystem {
    files: [File; 1]  
}
impl FileSystem {
    pub fn init() -> Self {
        Self {
            files: [File::init(0, -1, (0, 100)); 1]
        }
    }

    pub fn create_file(&self) {

    }
}