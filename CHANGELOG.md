<a name=""></a>
##  (2019-11-01)


#### Features

* **Logging:**  add support for scoped, async logging ([da9395e2](https://github.com/omarabid/rust-starter/commit/da9395e24c26b99977f09ec26742d8966232cfee))
* **clap:**  Basic Clap integration. ([02d14410](https://github.com/omarabid/rust-starter/commit/02d14410bc437c06014f1d829e7c1c156d6d07e6))
* **config:**
  *  implement global config with RwLock ([b5cf5ef8](https://github.com/omarabid/rust-starter/commit/b5cf5ef8eec1e76d0b13f9beac5b61100a8ec4a9))
  *  initial config implementation ([21c0f7be](https://github.com/omarabid/rust-starter/commit/21c0f7bed2173719749394917598d720ee1ee7f6))
* **docker:**  Initial Docker integration ([6c84c7d2](https://github.com/omarabid/rust-starter/commit/6c84c7d232b58e7036f9b4e6b4e4feacac2f5105))
* **hazard:**  Add a sample command ([e6380f4b](https://github.com/omarabid/rust-starter/commit/e6380f4b5acbc2332735581aa9aafb22338d9418))
* **human_panic:**  Add human_panic library ([2cef587e](https://github.com/omarabid/rust-starter/commit/2cef587e64947ffb143ff3afb006dc019ba52483))
* **logging:**  implement slog_syslog logging ([73ba1bf4](https://github.com/omarabid/rust-starter/commit/73ba1bf4541661da55e7969c1718034e070d5776))

#### Improvements

* **Cargo:**  initialize Rust library ([b974094f](https://github.com/omarabid/rust-starter/commit/b974094f73967178f16defa224853e8f7bad2792))
* **config:**
  *  refactor AppConfig to allow get/set for a single property ([64da0c58](https://github.com/omarabid/rust-starter/commit/64da0c5802013c4e57ace611d023d3ec3d8138b0))
  *  beautify the config output ([b0bb7004](https://github.com/omarabid/rust-starter/commit/b0bb7004650a4a5d878501177eb87da9feea5bf1))
* **core:**  add error conversion for slog-syslog ([0d00c2cc](https://github.com/omarabid/rust-starter/commit/0d00c2cc99d02b4868b19ab5f8812a1ecf6974f1))
* **dependencies:**  replace serde_derive crate by serde with derive feature ([13f8077b](https://github.com/omarabid/rust-starter/commit/13f8077b765ab5835dedc8aa19b2ac960800b831))
* **error:**
  *  add a sample error ([fd1a8805](https://github.com/omarabid/rust-starter/commit/fd1a88050391774927d29f1345f6a190a81c2412))
  *  Initial implementation for error.rs ([df931484](https://github.com/omarabid/rust-starter/commit/df9314848f14e8c71643703d982ed93398c47cf1))
* **human_panic:**  only include human panic in release builds ([75910c6a](https://github.com/omarabid/rust-starter/commit/75910c6afb614720c5dc692694f4b1bce46f4895))
* **tests:**  remove unwrap from tests ([ae070a8d](https://github.com/omarabid/rust-starter/commit/ae070a8d4f35a31b07ba3b7e14024e1499a7e46a))

#### Bug Fixes

* **config:**  make config properties visible ([b8499e9a](https://github.com/omarabid/rust-starter/commit/b8499e9a089490c862fb7ae59e1b5e0e2ae9a2f8))
* **error:**  add error chaining for AppConfig init ([09fc01b3](https://github.com/omarabid/rust-starter/commit/09fc01b3f386691e85752f6ea45b073c9a41e5f4))
* **slog:**  Temporarily disable logging ([c7c48b80](https://github.com/omarabid/rust-starter/commit/c7c48b80ddba7daefb072a8761b4a5691a1e40f0), breaks [#](https://github.com/omarabid/rust-starter/issues/))

#### Documentation

* **CONTR:**  Brief update to CONTRIBUTING.md ([81b114c8](https://github.com/omarabid/rust-starter/commit/81b114c859054bd83c9db6200448942d6f4296e8))
* **CONTRIBUTING.md:**  first iteration ([6fddae33](https://github.com/omarabid/rust-starter/commit/6fddae337a3b30799c6814165fb13b888df5c56f))
* **ISSUE_TEMPLATE.md:**  First iteration ([f02d0772](https://github.com/omarabid/rust-starter/commit/f02d07725229463ade2e8d81be1c73b79ef17530))
* **README:**  A brief update for README ([f3c02033](https://github.com/omarabid/rust-starter/commit/f3c02033572d051cd824b706950b5bacf8e09475))
* **badge:**
  *  Gitter Community badge ([19b899bb](https://github.com/omarabid/rust-starter/commit/19b899bb50e7c03dde14131d5fad1c049aae8f79))
  *  Circleci Badge ([aa6119c1](https://github.com/omarabid/rust-starter/commit/aa6119c1b92bdb7040b3f6df3d5f91c39e48ebe0))
* **comments:**  add a few random comments ([5c28f8e9](https://github.com/omarabid/rust-starter/commit/5c28f8e981e36e2fa0d1496922c271ddcd68749b))

#### Breaking Changes

* **slog:**  Temporarily disable logging ([c7c48b80](https://github.com/omarabid/rust-starter/commit/c7c48b80ddba7daefb072a8761b4a5691a1e40f0), breaks [#](https://github.com/omarabid/rust-starter/issues/))



