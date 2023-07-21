# common

This repository contains Rust libraries that are shared across Amphitheatre
components and libraries. They are considered internal to Amphitheatre, without
any stability guarantees for external usage.

- [**client**](src/client/): Represents the Rust client for the API.
- [**config**](src/config/): Common configuration structures.
- [**docker**](src/docker/): Docker container & Registry wrappers.
- [**filesystem**](src/filesystem/): File system related structures and helpers.
- [**schema**](src/schema/): The core schema includes manifest, playbook, and workspace.
- [**scm**](src/scm/): Provides a unified interface to multiple source code management systems.
- [**sync**](src/sync/): File synchronization structures and helpers.
- [**utils**](src/utils/): A variety of miscellaneous tools or functions.

## License

Copyright 2023 The Amphitheatre Authors. All rights reserved.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

      https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
