# cds-cli

Small cli to look at Hyperledger Fabric .cds files

*Note: this project was only created for something to experiment with while learning Rust!*

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

You can also unpack files from a .cds file using [7-Zip](https://www.7-zip.org)
