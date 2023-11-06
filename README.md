# Vue-rust project  
[![ci](https://github.com/uberballo/vue-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/uberballo/vue-rust/actions/workflows/ci.yml)
### What  
Simple fullstack project with `Vue` frontend and `Rust` backend. Allows the user to upload pictures, find the most common colors and invert the picture. There are still many areas to improve on, more on the [To do section](#to-do) 

### How  
On the backend we use [Axum](https://github.com/tokio-rs/axum) and [Image](https://docs.rs/image/latest/image/) and on the frontend `TS` + `Vue`. On the frontend we use `Axios` to send (Only .Jpg and .Png for now.) a data url to the backend where we decode the Base64 data to an image. Technically other image formats should work, but aren't tested.


### How to run  
Easiest way is by using Docker. Run the following:  
``` docker compose up -D```   
After everything is up and running, you can access the service on `localhost`. 

#### Setup
If you want to run them without docker, you need [Rust](https://www.rust-lang.org/tools/install) and [Node.js](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm). Tested with `rustc 1.73.0` and `Node v21.1.0`.  
* First install frontend dependencies with `npm i` in `vue-rust/front`.  

#### Start locally
After you've finished the necessary setup do the following to start locally: 
* Start the backend by running `cargo run` in `vue-rust/back` directory.  
* Start the development service with `npm run dev` on the `vue-rust/front` directory.  

### Testing  
After setup, run the following to run the tests: 
* Start the backend unit tests by running `cargo test` in `vue-rust/back` directory.  
* Start the frontend unit tests with `npm run test:unit` on the `vue-rust/front` directory.  

### To do  
If we'd like to achieve a production-quality software, we need to add and/or update quite a bit. 

- [ ] E2E-tests.  
- [X] CI pipeline.
- [ ] CD pipeline.
- [ ] Improved tracing and logging.  
- [ ] Performance tests.  
- [ ] Authorization.  
- [ ] Peer review :^)  
- [ ] Improve error handling, better error responses.  
- [ ] And maybe deployment in the future.  
