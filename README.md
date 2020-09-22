# cds-cli

Simple tool to manipulate Hyperledger Fabric .cds files

*Note: this project is unsupported and was only created for something to experiment with while learning Rust!*

## Downloads

Prebuilt binaries for Linux, OSX, and Windows can be downloaded from the latest release on the [releases page](https://github.com/jt-nti/cds-cli/releases).  

## Usage

Type `cds --help` to print help information.

## Example

To list the files inside a .cds file:

```
cds -x mysterious.cds | tar -tvf -
```

## Alternatives

The [configtxlator](https://hyperledger-fabric.readthedocs.io/en/release-2.0/commands/configtxlator.html) command can be used to decode .cds files. For example, to list the files inside a .cds file:

```
configtxlator proto_decode --type protos.ChaincodeDeploymentSpec --input mysterious.cds --output mysterious.json
jq -j .code_package mysterious.json | base64 --decode | tar -tvf -
```

There is another [cdstool](https://github.com/btl5037/cdstool) which you can install using `go get github.com/btl5037/cdstool` assuming you have Go installed and `$GOPATH/bin` on your path

You can also unpack files from a .cds file using [7-Zip](https://www.7-zip.org)
