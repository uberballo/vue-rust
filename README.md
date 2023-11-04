# Vue-rust project  

### What  
Simple fullstack project with `Vue` frontend and `Rust` backend. Allows the user to upload pictures, find the most common colors and invert the picture. There are still many areas to improve on, more on the [To do section](#to-do) 

### How  
On the backend we use [Axum](https://github.com/tokio-rs/axum) and [Image](https://docs.rs/image/latest/image/) and on the frontend `TS` + `Vue`. On the frontend we use `Axios` to send (Only Jpg for now) a data url to the backend where we decode the Base64 data to an image. 


### How to run  
TODO


### To do  
If we'd like to achieve a production-quality software, we need to add and/or update quite a bit. 

- [ ] E2E-tests.  
- [ ] CI/CD pipeline.
- [ ] Improved tracing and logging.  
- [ ] Performance tests.  
- [ ] Authorization.  
- [ ] Peer review :^)  
- [ ] Improve error handling, better error responses.  
- [ ] And maybe deployment in the future.  