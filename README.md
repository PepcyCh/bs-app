# Simple B/S App

Course project for Zhejiang University 2020-2021 Spring-Summer, Software Design of B/S Architecture.

We are asked to build a web app with both frontend and backend. It can receive and store MQTT messages sent by another Java program given by our teacher. Users can register and login to 'follow' some devices and view messages about those devices.

Frontend and backend are both written in Rust (frontend: [Yew](https://github.com/yewstack/yew), backend: [actix-web](https://github.com/actix/actix-web)) and MongoDB is used to store data.

### Pre-requisites

Before building your project, you need to make sure that all of the following are installed.
- Rust language
  via the official website or curl or other means

- trunk command line tool
  After installing Rust, use `cargo install trunk` takes up to 15minutes.

- wasm-bindgen-cli command line tool
  After installing Rust, use `cargo install wasm-bindgen-cli`  takes up to 10 minutes

- Node.js

- The rollup command-line tool
  After installing Node.js, install following. 

  ```shell
  npm install -g rollup
  npm install rollup/plugin-babel 
  npm install @rollup/plugin-node-resolve --save-dev
  ```

### Build project

#### Frontend

If you are building on Windows, you need to  manually enter the */js* folder and run `rollup -c`  // --config or -c to use *rollup.config.js* . then a *bundle.js* will be generated in the /dist folder

Then Run `trunk build --release` in the root of the frontend project (/frontend), and the build results will be in the /dist folder

#### Backend

Execute `cargo build --release` in the backend project root directory.
The result of the build is under the target/release folder.

### Deployment

We could choose Nginx for deployment and configure the static content (.html, .js, .wasm, .css, .ico) files in the front-end build results directory. Choose /dist for the configuration.

change nginx-1.21.6\conf\nginx.conf 

```
server {

    listen    9000;
    
        location / {
            root  xxx\bs-app\frontend\dist;
            index  index.html index.htm;
        } 
```

Configure the dynamic content, i.e., API requests, to the address configured on the backend, which is http://localhost:9000 by default, and then **start Nginx and the backend** to access it, where the backend can be run either by executing the generated executable directly, or by executing cargo run --release in its project root directory. in the project root directory.



