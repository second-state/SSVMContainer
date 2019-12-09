extern crate dirs;

use std::io::Read;
use std::io::Write;

#[derive(Debug)]
/// A file object which facilitates CRUD style operations on the file system
/// The default storage path is the calling user's home directory
pub struct FileSystem {
    base_dir: String
}

impl FileSystem {
    /// # Name 
    /// init
    /// # Purpose
    /// Initialises a SSVMContainer file system object
    /// # Input
    /// N/A
    /// # Returns
    /// FileSystem object
    /// # Example
    /// let fs = ssvm_container::storage::file_system::FileSystem::init();
    pub fn init() -> FileSystem {
    	let home_dir = dirs::home_dir();
    	let home_dir_string: String = home_dir.unwrap().to_str().unwrap().into();
        FileSystem{base_dir: home_dir_string}
    }

    /// # Name 
    /// create_application
    /// # Purpose
    /// Store Wasm on the file system so that it's funtions can be executed later
    /// # Input
    /// An instance of the FileSystem object
    /// A complete .wasm file in string format
    /// # Returns
    /// A unique identifier which can be used to access the application later
    /// # Example
    /// let bytecode_wasm = String::from("0x1234567890");
    /// let uuid = ssvm_container::storage::file_system::FileSystem::create_application(&fs, &bytecode_wasm);
    pub fn create_application(&self, _bytecode_wasm: &str) -> String {
    	let uuid = uuid::Uuid::new_v4().to_string();
    	let mut path = std::path::PathBuf::from(&self.base_dir);
    	path.push(&uuid);
    	std::fs::create_dir_all(path.as_path()).unwrap();
    	path.push("bytecode.wasm");
    	let mut file = std::fs::File::create(path.as_path()).unwrap();
    	file.write_all(_bytecode_wasm.as_bytes());
    	uuid
    }

    /// # Name 
    /// read_application
    /// # Purpose
    /// Read the Wasm of a given application
    /// # Input
    /// An instance of the FileSystem object
    /// An application's uuid as a string
    /// # Returns
    /// A complete .wasm file's contents, as a string
    /// # Example
    /// let bytecode_wasm_string = ssvm_container::storage::file_system::FileSystem::read_application(&fs, &uuid);
    pub fn read_application(&self, _application_uuid: &str) -> String {
        let mut reading_path = std::path::PathBuf::from(&self.base_dir);
        reading_path.push(&_application_uuid);
        reading_path.push("bytecode.wasm");
        let mut file_to_read = std::fs::File::open(reading_path.as_path()).unwrap();
        let mut buffer = Vec::new();
        let bytecode_wasm_string = file_to_read.read_to_end(&mut buffer).unwrap();
        bytecode_wasm_string.to_string()
    }

    /// # Name 
    /// update_application
    /// # Purpose
    /// Update the Wasm of a given application (overrides existing .wasm file)
    /// # Input
    /// An instance of the FileSystem object
    /// A complete .wasm file in string format
    /// # Returns
    /// A unique identifier which can be used to access the application later
    /// # Example
    /// ssvm_container::storage::file_system::FileSystem::update_application(&fs, &uuid, &bytecode_wasm_update);
    pub fn update_application(&self, _application_uuid: &str, _bytecode_wasm: &str) -> String {
        let mut update_path = std::path::PathBuf::from(&self.base_dir);
        update_path.push(&_application_uuid);
        update_path.push("bytecode.wasm");
    	let _file_to_update = std::fs::remove_file(update_path.as_path()).unwrap();
    	let mut file_to_write = std::fs::File::create(update_path.as_path()).unwrap();
    	file_to_write.write_all(_bytecode_wasm.as_bytes());
    	_application_uuid.to_string()
    }

   /// # Name 
    /// delete_application
    /// # Purpose
    /// Delete the application from the file system alltogether
    /// # Input
    /// An application's uuid as a string
    /// # Returns
    /// The application uuid of the application that was deleted
    /// # Example
    /// ssvm_container::storage::file_system::FileSystem::delete_application(&fs, &uuid);
    pub fn delete_application(&self, _application_uuid: &str) -> String {
    	let mut path = std::path::PathBuf::from(&self.base_dir);
    	path.push(&_application_uuid);
    	std::fs::remove_dir_all(path.as_path()).unwrap();
    	_application_uuid.to_string()

    }
}