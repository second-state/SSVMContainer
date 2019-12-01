# SSVMContainer

Users on the web/network are able to deploy and execute Wasm applications on SecondState's stack-based, stateless, WebAssembly Virtual Machine (SSVM). This SSVMContainer application sits between incoming requests from the external network (handled by [SSVMRPC](https://github.com/second-state/SSVMRPC)) and the SSVM. 

![architecture](https://github.com/second-state/SSVMRPC/blob/master/architecture.jpg)

Specifically, this SSVMContainer application handles the deployment of Wasm applications and also manages the execution of services (callable functions inside the Wasm application). The actual execution takes place inside the SSVM. However the execution is initiated by this container and all of the application state information is handled by this SSVMContainer.

# Storage

## File system

At present this SSVMContainer simple uses the file system.
![storage file system](https://github.com/second-state/SSVMContainer/blob/master/storage_file_system.jpg)

## LevelDB

Future versions will allow the storage to be configured for LevelDB also.
