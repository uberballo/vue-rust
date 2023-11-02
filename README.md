# Vue-rust project  

### What  
Simple fullstack project with `Vue` frontend and `Rust` backend. Allows the user to upload pictures, find the most common colors and invert the picture.  

### How  
On the backend we use [Axum](https://github.com/tokio-rs/axum) and [Image](https://docs.rs/image/latest/image/) and on the frontend `TS` + `Vue`. On the frontend we use `Axios` to send (Only Jpg for now) a data url to the backend where we decode the Base64 data to an image. 


### How to run  
TODO