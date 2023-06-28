# TODO

- [ ] Core
  - [ ] Listener
    - [ ] with builder design pattern for building a customizable listener server on top of TokIO
  - [ ] Parser
    - [ ] Parse Request 
      - [ ] PreRequest Builder
        - [ ] Request
      - [ ] Parse Response
       - [ ] Response Builder
         - [ ] Response
  - [ ] Responder: trait for implment responses look for example `impl Responder for String`

- [ ] API
  - [ ] this is an important thing about Behelit APIs is a trait I need to think about it 

- [ ] Proto Note: after Core Todos
  - [ ] http1 
  - [ ] http2
  - [ ] http3

- [ ] tip: Request/Response 
  - [ ] write request parser fast and lightweight with the lowest count of copy and clone to make it look like no std. it means to try to don't use the String type.

- [ ] Tests
  - [ ] Test Request
  - [ ] Test Parser
  - [ ] Test Response
  - [ ] Test Responder
  - [ ] Test Listener

- [x] Clean
  - [x] Cargo.toml: remove additional packages and features

- [ ] Feat
  - [ ] Add impl Diesel Error to fix error response
  - [ ] Add Log System
  - [ ] Add Middleware
  - [ ] Add Guard
