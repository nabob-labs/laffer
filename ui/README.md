# UI

Web interface for accessing [Velox](../velox/)

## Packages

| Packages                                         | Description                                                                                                                         |
| ------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------- |
| [`applets-kit`](./applets/kit)                   | A common library for developing applets and communicate with velox portal                                                           |
| [`config`](./config)                             | Common configurations for ui elements such as tailwind, fonts, etc...                                                               |
| [`store`](./store)                               | It allows connect with velox blockchain, connect multiples wallets, manages accounts, and enables interaction with smart contracts. |
| [`proxy`](./workers/proxy)                       | Cloudflare worker used as proxy for devnet rpc                                                                                      |
| [`webrtc-signaling`](./workers/webrtc-signaling) | A WebRTC signaling server used to establish peer-to-peer connections between clients.                                               |

----

## Apps

| Apps                                    | Description                                       |
| --------------------------------------- | ------------------------------------------------- |
| [`portal-website`](./ui/portal/website) | Velox portal website                              |
| [`portal-app`](./ui/portal/app)         | Velox portal react native app for Android and iOS |

----

## Applets

| Applet | Description |
| ------ | ----------- |
