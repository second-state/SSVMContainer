extern crate dirs;
extern crate rand;

use std::{thread, time};

use std::process::Command;
use std::io::{self};

use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::io::BufReader;



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

fn does_file_exist(_file_path: &str) -> bool{
    let output_json_path = std::path::PathBuf::from(_file_path);
    let mut retry_count: i32 = 0;
    let mut the_response: bool = false;
    let mut done: bool = false;
    while !done{
        retry_count = retry_count + 1;
        if output_json_path.exists(){
            the_response = true;
            done = true;
        } else if retry_count >=40{
            the_response = false;
            done = true;
        }
        thread::sleep(time::Duration::from_millis(250));
        println!("Checking to see if file called: {:?} exists", &_file_path);
    }
        return the_response;
}

fn get_current_vmsnapshot(_output_dir: String) -> io::Result<String> {
        // This blank vm_snapshot will be returned if a real/current one is not located during this function's operation
        let mut json_return_value: Value = json!({"vm_snapshot": {}});
        // Obtain the current VMSnapshot, if one exists
        println!("Scanning output directory at {:?} for latest vm_snapshot ...", _output_dir);
        let mut entries = fs::read_dir(_output_dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
        if entries.len() > 0 {
            println!("Sorting timestamp dirs");
            entries.sort();
            for x in (0..entries.len()).rev() {
                println!("Processing dir at position {:?}", x);
                if entries[x].is_dir() {
                    let tsv = entries[x].clone();
                    let file_path_string: String = String::from(tsv.into_os_string().into_string().unwrap());
                    println!("Most recent timestamp directory is: {:?}", file_path_string);
                    // Open and read the VMSnapshot section of the output.json file
                    let mut snapshot_file_path = std::path::PathBuf::from(&file_path_string); 
                    snapshot_file_path.push("output");
                    snapshot_file_path.set_extension("json");
                // If there is a output.json file with vm_snapshot data it will make up the return value of this function
                    if snapshot_file_path.exists(){
                        println!("Reading most recent output.json file to obtain vm_snapshot");
                        let snapshot_file_handle = File::open(snapshot_file_path);
                        let output_reader = BufReader::new(snapshot_file_handle.unwrap());
                        let mut whole_file = serde_json::from_reader::<_, serde_json::Value>(output_reader)?;
                        // TODO THIS JSON RETURN VALUE IS WORKING, NEED TO PARSE OUT THE DATA AND SEND THAT BACK
                        json_return_value = serde_json::from_value(whole_file["result"]["vm_snapshot"].take())?;
                        //println!("vm_snapshot is as follows: {:?}", serde_json::to_string(&json_return_value).unwrap());
                        println!("vm_snapshot is as follows: {:?}", &json_return_value).unwrap();
                    // TODO Extract the vm_snapshop JSON only
                    // TODO Save that JSON as a return_string

                    }
                    break;
                } else {
                    println!("Skipping this entry because it is not a directory ...");
                }

            }
        } 
        // Perform the return
        println!("Returning: {:?}", serde_json::to_string(&json_return_value).unwrap());
        Ok(serde_json::to_string(&json_return_value).unwrap())
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
    	//let uuid = uuid::Uuid::new_v4().to_string();
        let y: u64 = rand::random::<u64>();
        let p = String::from("0x");
        let uuid: String = format!("{}{:x}", p, y);
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
    /*
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
    pub fn execute_ewasm_function(&self, _uuid: &str,  _modules: &Value, _function_name: &str, _function_arguments: &Value) -> String {
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
    */
    
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
    pub fn execute_wasm_function(&self, _uuid: &str, _modules: &Value, _function_name: &str, _function_arguments: &Value, _argument_types: &Value, _return_types: &Value) -> String {
        // New timestamp
        let timestamp_value = &self.get_time_in_seconds();
        

        // Output json path
        let mut output_json_path = std::path::PathBuf::from(&self.base_dir);
        output_json_path.push(&_uuid);
        let output_directory = output_json_path.clone();
        output_json_path.push(timestamp_value);
        output_json_path.push("output");
        output_json_path.set_extension("json");
        let ojp = output_json_path.clone();
        let ojp2 = output_json_path.clone();
        let ojp3 = output_json_path.clone();
        //let output_json_path_as_string = String::from(output_json_path.as_path());
        println!("Output json path: {:?}", output_json_path);
        

        // Obtain the current VMSnapshot from the current timestamp dir (if one exists)
        let current_vm_snapshot = get_current_vmsnapshot(output_directory.into_os_string().into_string().unwrap());
        println!{"Current vm_snapshot: {:?}", current_vm_snapshot};

        // Bytecode path
        let mut bytecode_path = std::path::PathBuf::from(&self.base_dir);
        bytecode_path.push(&_uuid);
        bytecode_path.push("bytecode");
        bytecode_path.set_extension("wasm");
        let bp = bytecode_path.clone();
        //let bytecode_path_as_string = String::from(bytecode_path);
        println!("Bytecode path: {:?}", bytecode_path);
        

        // Input json path
        let mut input_json_path = std::path::PathBuf::from(&self.base_dir);
        input_json_path.push(&_uuid);
        input_json_path.push(timestamp_value);
        // Create time stamp directory
        std::fs::create_dir_all(input_json_path.as_path()).unwrap();
        input_json_path.push("input");
        input_json_path.set_extension("json");
        //let input_json_path_as_string = String::from(input_json_path.as_path());
        println!("Input json path: {:?}", input_json_path);

        // Create the contents for the input json file
        let mut service_name: String = String::from("");
        service_name = format!("{}_{}_{}", _uuid, timestamp_value, _function_name);
        let input_json = json!({"service_name": service_name ,"uuid": _uuid,"modules": _modules,"execution": {"function_name": _function_name,"argument": _function_arguments, "argument_types": _argument_types, "return_types": _return_types, "vm_snapshot": current_vm_snapshot.unwrap()}});
        // Convert the input json object to a string for writing to the file
        let input_json_as_string = serde_json::to_string(&input_json);
        // Write the contents to the input json file
        let ijp = input_json_path.clone();
        let writer = BufWriter::new(File::create(input_json_path).unwrap());
        serde_json::to_writer_pretty(writer, &input_json).unwrap();
        

        // Build the command as a Command object and call SSVM directly
        Command::new("ssvm-proxy").arg("--input_file").arg(ijp.into_os_string()).arg("--output_file").arg(ojp.into_os_string()).arg("--bytecode_file").arg(bp.into_os_string()).spawn().expect("Please ensure that ssvm-proxy is in your system PATH");
        println!("SSVM command has been executed, please wait ...");
        

        // Check to see if output has been written
        if does_file_exist(&ojp2.into_os_string().into_string().unwrap()) == true {
            let output_file_handle = File::open(&ojp3);
            let output_reader = BufReader::new(output_file_handle.unwrap());
            let return_value = serde_json::from_reader(output_reader).unwrap();
            // Return results
            return return_value;
        } else {
            let error_string: String = String::from("Output file does not exist.");
            return error_string;
        }
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
