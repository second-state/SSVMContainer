extern crate ssvm_container;
use serde_json::Value;

fn main() {
	// Example of how to initiate an instance of a file system object
    let fs = ssvm_container::storage::file_system::FileSystem::init();
    println!("{:?}", fs);

    // Example of how to create an application
    let bytecode_wasm = String::from("0x1234567890");
    let application_name = String::from("Application 1");
    let uuid = ssvm_container::storage::file_system::FileSystem::create_application(&fs, &bytecode_wasm, &application_name);
    println!("Application has a uuid of: {:?}", uuid);

    // Example of how to read an application 
    let v: Value = serde_json::from_str(&uuid).unwrap();
    let application_uuid = &v["response"]["application"]["uuid"].as_str();
    println!("Application name: {:?}", application_uuid);
    let bytecode_wasm_string = ssvm_container::storage::file_system::FileSystem::read_application(&fs, application_uuid.unwrap());
    println!("Bytecode wasm string: {:?}", bytecode_wasm_string);

    // Example of how to update an application's stored .wasm
    //let bytecode_wasm_update = String::from("0x8888888888");
    //ssvm_container::storage::file_system::FileSystem::update_application(&fs, &uuid, &bytecode_wasm_update);

    // Example of how to remove/delete an application
    ssvm_container::storage::file_system::FileSystem::delete_application(&fs, application_uuid.unwrap());
}
