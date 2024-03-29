
# PatchMe (beta)



A simple patch generation tool implemented in Rust using Cargo. This tool generates a Git-compliant unified diff patch file that represents the differences between two files.



## Features



- Read original and modified files from user input.

- Calculate differences between the original and modified content.

- Generate a Git-compliant unified diff patch file.



## Prerequisites



- Rust (Install from [rustup.rs](https://rustup.rs/))

- Cargo (Included with Rust)



## Installation



1. Clone the repository:



```bash

git clone https://github.com/hamzahasbi/patchme

cd patchme

cargo build --release

```



## Usage

* Run the compiled executable:


```bash
./target/release/patchme
````
When prompted, enter the paths to the original and modified files and the tool will generate a patch file in the project directory, which contains the diff file.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License. See the [License](https://github.com/hamzahasbi/patchme/blob/main/LICENSE) file for details.
