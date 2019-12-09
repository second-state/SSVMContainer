# SSVMContainer

Users on the web/network are able to deploy and execute Wasm applications on SecondState's stack-based, stateless, WebAssembly Virtual Machine (SSVM). This SSVMContainer library conceptually sits between incoming requests from the external network (handled by [SSVMRPC](https://github.com/second-state/SSVMRPC)) and the [SSVM](https://github.com/second-state/SSVM). At present, SSVMContainer's public module items are available and its functionality can be implemented by the calling party i.e. by an RPC server application like [SSVMRPC](https://github.com/second-state/SSVMRPC). The ultimate design goal is to implement an Inversion of Control (IoC) design and for SSVMContainer to be a conduit between a variety of interfaces (not just RPC). Please see the roadmap section below for more information about IoC work.

![architecture](https://github.com/second-state/SSVMRPC/blob/master/architecture.jpg)

Specifically, this SSVMContainer application handles the deployment of Wasm applications and also manages the execution of services (callable functions inside the Wasm application). The actual execution takes place inside the SSVM. However the execution is initiated by this container and all of the application state information is handled by this SSVMContainer.

# Storage

## File system

At present this SSVMContainer simple uses the file system.
![storage file system](https://github.com/second-state/SSVMContainer/blob/master/storage_file_system.jpg)

## LevelDB

Future versions will allow the storage to be configured for LevelDB also. Once this transition happens, the following JSON (which reflects the above file-system layout) will be used.

```
{
	"application_uuid": "0x11111111",
	"application_name": "ERC 20",
	"bytecode": "0x99999999",
	"service": {
		"service_uuid": "0x11111111",
		"service_name": "add",
		"timestamp": {
			"timestamp_uuid": "1575158141",
			"input": {},
			"output": {}
		}
	}
}
```

# Roadmap

This 
