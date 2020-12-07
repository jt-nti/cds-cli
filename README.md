# cds-cli

Simple tool to manipulate Hyperledger Fabric .cds files

*Note: this project is unsupported and was only created for something to experiment with while learning Rust!*

## Downloads

Prebuilt binaries for Linux, OSX, and Windows can be downloaded from the latest release on the [releases page](https://github.com/jt-nti/cds-cli/releases).

For example, to install on linux:

```
curl -L "https://github.com/jt-nti/cds-cli/releases/download/0.5.0/cds-0.5.0-linux" -o cds && chmod u+x cds
```

## Usage

Type `cds --help` to print help information.

## Examples

### View .cds file contents

To list the files inside a .cds file:

```
cds -x mysterious.cds | tar -tvf -
```

### Create new .cds file

You will need a .tgz file containing the chaincode you want to package.
The chaincode **must** be in a _src/_ directory.
For example, assuming you have cloned the [fabric-samples](https://github.com/hyperledger/fabric-samples) repository:

```
tar -czf "fabcarcc.tgz" --transform s/go/src/ -C ./fabric-samples/chaincode/fabcar/ go
```

This should create a `fabcarcc.tgz` file with the following contents:

```
src/
src/fabcar.go
src/go.mod
src/go.sum
```

Then to create a .cds file:

```
./cds --create --lang golang --name fabcarcc --version 1 --module github.com/hyperledger/fabric-samples/chaincode/fabcar/go --output fabcarcc.cds fabcarcc.tgz
```

**Note:** the `--module` option is only required for golang chaincode and must match the module defined in the `go.mod` file

## Alternatives

### View .cds file contents

The [configtxlator](https://hyperledger-fabric.readthedocs.io/en/release-2.0/commands/configtxlator.html) command can be used to decode .cds files. For example, to list the files inside a .cds file:

```
configtxlator proto_decode --type protos.ChaincodeDeploymentSpec --input mysterious.cds --output mysterious.json
jq -j .code_package mysterious.json | base64 --decode | tar -tvf -
```

There is another [cdstool](https://github.com/btl5037/cdstool) which you can install using `go get github.com/btl5037/cdstool` assuming you have Go installed and `$GOPATH/bin` on your path

You can also unpack files from a .cds file using [7-Zip](https://www.7-zip.org)

### Create new .cds file

Use the `peer chaincode package` command. For example:

```
peer chaincode package fabcarcc.cds --lang golang --name fabcarcc --version 1 --path ./fabric-samples/chaincode/fabcar/go
```
