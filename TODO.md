# TODO

- [X] setup run to run on the browser
    - [ ] setup run to call on functions from rust code
- [ ] setup the client to run datafusion on csv data
    - can not run datafusion in the client since it uses tokio under the hood to process the files
- [ ] setup a server of the executable which connects to ipfs
    - [ ] setup client to get data from server which connects to ipfs
    - [ ] setup client to run datafusion in the server and return the results
- [ ] An alternative to datafusion is polars which has great support for running in wasm

# Issues
- [ ] Can not run datafusion in wasm
- [ ] Can not run polars in wasm
