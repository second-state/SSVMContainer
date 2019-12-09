# SSVMContainer

Users on the web/network are able to deploy and execute Wasm applications on SecondState's stack-based, stateless, WebAssembly Virtual Machine (SSVM). This SSVMContainer library conceptually sits between incoming requests from the external network (handled by [SSVMRPC](https://github.com/second-state/SSVMRPC)) and the [SSVM](https://github.com/second-state/SSVM). At present, SSVMContainer's public module items are available and its functionality can be implemented by the calling party i.e. by an RPC server application like [SSVMRPC](https://github.com/second-state/SSVMRPC). The ultimate design goal is to implement an Inversion of Control (IoC) design and for SSVMContainer to be a conduit between a variety of interfaces (not just RPC). Please see the [roadmap section below](https://github.com/second-state/SSVMContainer/blob/master/README.md#roadmap) for more information about IoC work.

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

As mentioned above, the ultimate goal of SSVMContainer is to take on IoC by design. This is a slightly longer term goal than the current working prototype. The current SSVMContainer demonstrates how to manage multiple applications and their state in a stateless execution environment. As shown in the diagram above, the SSVMContainer takes HTTP POST requests from the web via SSVMRPC, then executes code on the stateless SSVM. The SSVMContainer not only passes back the results to SSVMRPC (so that the original HTTP POST sender gets a result), SSVMContainer also records all incoming and outgoing activity and the state of any and all of the applications at any given point in time. SSVMContainer even stores each applications Wasm bytecode (which is passed into SSVM during any of the stateless execution activities).

## IoC and Rust

As [this paper](http://cs242.stanford.edu/assets/projects/2017/diamondm-mvilim.pdf) explains, while IoC design patterns are quite common in other languages, they are difficult to implement in Rust, given Rust’s design philosophy. For example, using a factory-based method in particular suffers from low safety, limited flexibility, and verbosity.

From a dependency perspective the following statement is sound "depencency injection decouples your class's construction, from the construction of its dependencies". As Rust does not use classes, this can be done in a variety of ways i.e using traits and/or modules and even internal module functions (which facilitate both constructor injection and/or setter injection).

### Rust traits

In Rust, a [trait](https://doc.rust-lang.org/1.8.0/book/traits.html) is "a language feature that tells the Rust compiler about functionality which a type must provide". For example, in the absense of a traditional "Class" in Rust, a `struct` called "Circle" would hold only data fields such as `radius`. A `trait` called "HasArea" would hold only a type signature for the `area` of a given cicle. Finally the `impl HasArea for Circle` implementation of this would actually perform the calculating of the area of a circle. 

The [Substrate Developer Hub documentation](https://substrate.dev/docs/en/getting-started/using-the-substrate-scripts#substrate-module-new) shows the use of Rust traits i.e. creating a new runtime module via a [template](https://github.com/paritytech/substrate/blob/v1.0/node-template/runtime/src/template.rs). A handfull of Rust IoC experiments/projects on GitHub such as [ioc](https://github.com/qrlpx/ioc), [shaku](https://github.com/bgbahoue/shaku), [ioc-rs](https://github.com/fkoep/ioc-rs) and [injectorust](https://github.com/talhazengin/injectorust) all employ this use of traits.
