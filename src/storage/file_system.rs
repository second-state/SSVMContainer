extern crate dirs;

use std::io::BufWriter;
use std::io::BufReader;
use std::fs::File;

use std::io::Read;
use std::io::Write;
use serde_json::json;
use serde_json::Value;
use std::time::SystemTime;
/// A file object which facilitates CRUD style operations on the file system
/// The default storage path is the calling user's home directory
#[derive(Debug)]
pub struct FileSystem {
    base_dir: String
}

// Private code that facilitates the reading and writing of a name
fn write_name(_file_path: &std::path::PathBuf, _name: &str){
    let value = json!({"name": &_name});
    let mut path = std::path::PathBuf::from(_file_path);
    path.push("name.json");
    let writer = BufWriter::new(File::create(path).unwrap());
    serde_json::to_writer_pretty(writer, &value).unwrap();
}

fn read_name(_file_path: &str) -> String{
    let mut path = std::path::PathBuf::from(_file_path);
    path.push("name.json");
    let reader = BufReader::new(File::open(path).unwrap());
    let name = serde_json::from_reader(reader).unwrap();
    return name;
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
    
    fn get_time_in_seconds(&self) -> String{
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH){
            Ok(n) => return n.as_secs().to_string(),
            Err(_) => panic!("Error"),
        }
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
    pub fn create_application(&self, _bytecode_wasm: &str, _application_name: &str) -> String {
        // Create unique ID
    	let uuid = uuid::Uuid::new_v4().to_string();
        // Initialize a path
    	let mut path = std::path::PathBuf::from(&self.base_dir);
        // Extend the path
    	path.push(&uuid);
        // Create uuid as dir
    	std::fs::create_dir_all(path.as_path()).unwrap();
        // Create name as new json file
        write_name(&path, _application_name);
        // Extend path 
        path.push("bytecode");
        path.set_extension("wasm");
        // Create bytecode as file
        let mut file = std::fs::File::create(path.as_path()).unwrap();
        file.write_all(_bytecode_wasm.as_bytes());
        // return the uuid
    	let return_value = json!({"response":{"status": "success","application":{"uuid": uuid, "name": _application_name}}});
        let return_value_as_string = serde_json::to_string(&return_value);
        return return_value_as_string.unwrap();
    }

    /// # Name 
    /// execute_ewasm_function
    /// # Purpose - Execute Ethereum flavoured WebAssembly (Wasm) on SSVM
    /// Allow calling code to pass in the name of an Ethereum (Ewasm) function and its arguments so that it can be executed on SSVM, and the result returned
    /// # Input
    /// An instance of the FileSystem object
    /// The uuid of an existing application (wasm code that was deployed using this file's create_application function)
    /// The name of the function to be executed
    /// The arguments for the function
    /// Any modules that are required i.e. any special implementations of multimedia, graphics, file system which are supported on the SSVM
    /// # Returns
    /// The output from the SSVM
    /// Also stores
    /// let uuid = ssvm_container::storage::file_system::FileSystem::create_application(&fs, &bytecode_wasm);
    pub fn execute_ewasm_function(&self, _uuid: &str, _function_name: &str, _function_arguments: &Value, _modules: &Value) -> String {
        // Initialize a path
        let mut path = std::path::PathBuf::from(&self.base_dir);
        // Extend the path
        path.push(&_uuid);
        // Read in the available bytecode
        let bytecode_string = &self.read_application(&_uuid);
        println!("Application bytecode: {:?}", bytecode_string);
        println!("Function name: {:?}", _function_name);
        println!("Function arguments: {:?}", _function_arguments);
        println!("Modules: {:?}", _modules);
        // Get time
        &self.create_timestamp_dir();
        // Build the SSVM command as a string
        
        // result = ssvm_command, function_name, function_arguments, modules, bytecode
        //
        // Then call SSVM directly
        //
        // Return SSVM output
        //
        // Create fresh time stamp dir in this uuid dir then write input we used to input.json and the output we received to output.json

        let return_value = json!({"response":{"status": "success","application":{"uuid": _uuid}}});
        let return_value_as_string = serde_json::to_string(&return_value);
        return return_value_as_string.unwrap();
    }


    /// # Name 
    /// execute_wasm_function
    /// # Purpose
    /// Allow calling code to pass in the name of a function and its arguments so that it can be executed on SSVM, and the result returned
    /// # Input
    /// An instance of the FileSystem object
    /// The uuid of an existing application (wasm code that was deployed using this file's create_application function)
    /// The name of the function to be executed
    /// The arguments for the function
    /// Any modules that are required i.e. any special implementations of multimedia, graphics, file system which are supported on the SSVM
    /// # Returns
    /// The output from the SSVM
    /// Also stores
    /// let uuid = ssvm_container::storage::file_system::FileSystem::create_application(&fs, &bytecode_wasm);
    pub fn execute_wasm_function(&self, _uuid: &str, _function_name: &str, _function_arguments: &Value, _modules: &Value) -> String {
        // Initialize a path
        let mut path = std::path::PathBuf::from(&self.base_dir);
        // Extend the path
        path.push(&_uuid);
        // Read in the available bytecode
        let bytecode_string = &self.read_application(&_uuid);
        println!("Application bytecode: {:?}", bytecode_string);
        println!("Function name: {:?}", _function_name);
        println!("Function arguments: {:?}", _function_arguments);
        println!("Modules: {:?}", _modules);
        let timestamp_value = &self.get_time_in_seconds();
        path.push(&timestamp_value);
        std::fs::create_dir_all(path.as_path()).unwrap();
        // Build the SSVM command as a string

        // result = ssvm_command, function_name, function_arguments, modules, bytecode
        //
        // Then call SSVM directly
        //
        // Return SSVM output
        //
        // Create fresh time stamp dir in this uuid dir then write input we used to input.json and the output we received to output.json

        let return_value = json!({"response":{"status": "success","application":{"uuid": _uuid}}});
        let return_value_as_string = serde_json::to_string(&return_value);
        return return_value_as_string.unwrap();
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
        // Extend the path
        path.push(&_application_uuid);
        println!("Deleting path at: {:?}", path.as_path());
    	std::fs::remove_dir_all(path.as_path()).unwrap();
    	let return_value = json!({"response":{"status": "success","application":{"storage": "file_system", "uuid": _application_uuid}}});
        let return_value_as_string = serde_json::to_string(&return_value);
        return return_value_as_string.unwrap();
    }
}
