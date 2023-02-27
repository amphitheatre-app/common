# common

This repository contains Rust libraries that are shared across Amphitheatre
components and libraries. They are considered internal to Amphitheatre, without
any stability guarantees for external usage.

- [**client**](src/client/): Represents the Rust client for the API.
- [**config**](src/config/): Common configuration structures.
- [**docker**](src/docker/): Docker container & Registry wrappers.
- [**filesystem**](src/filesystem/): File system related structures and helpers.
- [**schema**](src/s): The core schema includes manifest, playbook, and workspace.
- [**scm**](src/scm): Provides a unified interface to multiple source code management systems.
- [**utils**](src/utils/): A variety of miscellaneous tools or functions.

## License

Licensed under the [Apache License 2.0](https://github.com/amphitheatre-app/common/blob/master/LICENSE)
