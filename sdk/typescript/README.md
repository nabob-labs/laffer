## Bolt and Velox SDK

You can find more info in our [documentation](https://bolt-sdk.pages.dev/).

## Packages

#### `/packages`

| Package                       | Description                                                                                                                                                 |
| ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`config`](./config)          | Common configurations for tsconfig, tsup, biome, typedoc                                                                                                    |
| [`velox`](./velox/)           | The SDK package provides a set of tools and utilities to interact with [Velox] chain                                                                        |
| [`crypto`](./bolt/crypto)     | It includes various cryptographic functions and utilities for encryption, decryption, hashing, and more.                                                    |
| [`encoding`](./bolt/encoding) | Encoding helpers that are used across packages and assist in encoding and decoding data in various formats                                                  |
| [`bolt-sdk`](./bolt)          | The SDK package provides a set of tools and utilities to interact with [Bolt] execution environment                                                         |
| [`types`](./bolt/types)       | It contains various type definition used across the codebase. These types help ensure type safety and improve code readbility.                              |
| [`utils`](./bolt/utils)       | Its a collection of utility functions that are used across the project. These utilities are designed to simplify common tasks and improve code reusability. |

## Supported JS environments

Packages in the workspace are compiled to JavaScript ES2022, targeting the latest ECMAScript standard, and support both ESM and CJS module formats.

1. Node.js 21+
2. Modern browsers (Chromium/Firefox/Safari)
3. Browser extensions (Chromium/Firefox)

## Acknowledgement

This project draws inspiration from and follows some of the architectural design principles of [Viem], while utilizing foundational code from [Wagmi]. Several concepts and ideas have been directly adapted from their codebase, significantly influencing this project.

Additionally, we would like to acknowledge [CosmJS] for providing essential code and tools that contributed to key aspects of this project.

We are grateful to both the [Wevm] and [Confio] team for their open-source contributions and the valuable support they offer to the community.

## License

TBD

[Bolt]: https://bolt.build/
[Velox]: ../../velox
[Wevm]: https://wevm.dev/
[Wagmi]: https://github.com/wevm/wagmi
[Viem]: https://github.com/wevm/viem
[CosmJS]: https://github.com/cosmos/cosmjs
[Confio]: https://confio.gmbh/
