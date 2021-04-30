# Simple B/S App

Course project for Zhejiang University 2020-2021 Spring-Summer, Software Design of B/S Architecture.

We are asked to build a web app with both frontend and backend. It can receive and store MQTT messages sent by another Java program given by our teacher. Users can register and login to 'follow' some devices and view messages about those devices.

Frontend and backend are both written in Rust (frontend: [Yew](https://github.com/yewstack/yew), backend: [actix-web](https://github.com/actix/actix-web)) and MongoDB is used to store data.

