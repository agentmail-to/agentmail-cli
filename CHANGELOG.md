# Changelog

## 0.7.7 (2026-03-31)

Full Changelog: [v0.7.6...v0.7.7](https://github.com/agentmail-to/agentmail-cli/compare/v0.7.6...v0.7.7)

### Features

* **api:** api update ([54154c0](https://github.com/agentmail-to/agentmail-cli/commit/54154c064b74dea0d84fc756e5e213a2d9373fc7))


### Bug Fixes

* add permissions for auto-merge workflow ([673e067](https://github.com/agentmail-to/agentmail-cli/commit/673e0673bca92ae94844c4070370d635475951eb))
* use --admin to bypass branch protection for release merge ([7e02df7](https://github.com/agentmail-to/agentmail-cli/commit/7e02df7c62342ea3fcd6b8906b32013153d3bb63))
* use --auto flag for release PR merge ([3034d63](https://github.com/agentmail-to/agentmail-cli/commit/3034d63a1892aad9c2704ae489183755e4f10842))

## 0.7.6 (2026-03-31)

Full Changelog: [v0.7.5...v0.7.6](https://github.com/agentmail-to/agentmail-cli/compare/v0.7.5...v0.7.6)

### Features

* **api:** manual updates ([1ea2031](https://github.com/agentmail-to/agentmail-cli/commit/1ea20312631ce0537767e2bb09a6515df9c1cdca))

## 0.7.5 (2026-03-30)

Full Changelog: [v0.7.4...v0.7.5](https://github.com/agentmail-to/agentmail-cli/compare/v0.7.4...v0.7.5)

### Features

* **api:** api update ([a4f320b](https://github.com/agentmail-to/agentmail-cli/commit/a4f320bc9c3458fa81caa585e9d6a9886e42d514))

## 0.7.4 (2026-03-28)

Full Changelog: [v0.7.3...v0.7.4](https://github.com/agentmail-to/agentmail-cli/compare/v0.7.3...v0.7.4)

### Bug Fixes

* add go.sum entry for agentmail-go v0.1.0 ([36f3175](https://github.com/agentmail-to/agentmail-cli/commit/36f3175a14c51a3bea17589fb7cae7e9cae50c1d))

## 0.7.3 (2026-03-28)

Full Changelog: [v0.7.2...v0.7.3](https://github.com/agentmail-to/agentmail-cli/compare/v0.7.2...v0.7.3)

### Features

* **api:** api update ([9d775cd](https://github.com/agentmail-to/agentmail-cli/commit/9d775cd3a27c5915bfc57576441026b0617b0fd6))


### Chores

* update agentmail-go to v0.1.0 ([9ed80ba](https://github.com/agentmail-to/agentmail-cli/commit/9ed80ba6f772e0a60f5da58cd56af25e23890e0a))

## 0.7.2 (2026-03-28)

Full Changelog: [v0.7.1...v0.7.2](https://github.com/agentmail-to/agentmail-cli/compare/v0.7.1...v0.7.2)

### Features

* add `--max-items` flag for paginated/streaming endpoints ([cab651f](https://github.com/agentmail-to/agentmail-cli/commit/cab651f16b51cc0b8fd8197307bd9a2a3149097f))
* **api:** api update ([09c687b](https://github.com/agentmail-to/agentmail-cli/commit/09c687b64f3429aa50735b0dd02727f269d9419f))
* **api:** api update ([8930431](https://github.com/agentmail-to/agentmail-cli/commit/8930431afd5c13e99e17d336b8bbf57da7c97b3a))
* **api:** api update ([93ec607](https://github.com/agentmail-to/agentmail-cli/commit/93ec607579dc3c74a00c1559876f19159f17bf18))
* **api:** api update ([922d06b](https://github.com/agentmail-to/agentmail-cli/commit/922d06ba6370e908121be1ce2a3dcb60a025a919))
* **api:** api update ([5978d7a](https://github.com/agentmail-to/agentmail-cli/commit/5978d7ab85c90ec624165671208606c5d3bb9825))
* **api:** api update ([212d624](https://github.com/agentmail-to/agentmail-cli/commit/212d6248a314de915377c0fa35c346e7b9fe3549))
* **api:** api update ([b3aa3b4](https://github.com/agentmail-to/agentmail-cli/commit/b3aa3b49b6b1077ba92aa4146f7860dff0b05920))
* set CLI flag constant values automatically where `x-stainless-const` is set ([4484b6f](https://github.com/agentmail-to/agentmail-cli/commit/4484b6f0c2b23f4b4e53a3ad7c2594ee7f37ef5e))
* support passing required body params through pipes ([5caad8a](https://github.com/agentmail-to/agentmail-cli/commit/5caad8a5272b56a28394d040333c420a9f853b56))


### Bug Fixes

* avoid reading from stdin unless request body is form encoded or json ([32f2697](https://github.com/agentmail-to/agentmail-cli/commit/32f26970e29bc6c1b02d2e543bda76b38d018d5a))
* better support passing client args in any position ([632158a](https://github.com/agentmail-to/agentmail-cli/commit/632158a4431957bc86e8f0bc1b33adefb7067e59))
* cli no longer hangs when stdin is attached to a pipe with empty input ([df22dd4](https://github.com/agentmail-to/agentmail-cli/commit/df22dd4c119b70b4b4dcdf1ee307a1524b40fb1f))
* fix for encoding arrays with `any` type items ([7898e52](https://github.com/agentmail-to/agentmail-cli/commit/7898e52f357d2232cb5db0c22791692dabc13d30))
* fix for off-by-one error in pagination logic ([d303204](https://github.com/agentmail-to/agentmail-cli/commit/d303204d3ace0fac1d099ad13dcb47b2fd64c4a8))
* fix for test cases with newlines in YAML and better error reporting ([d0fbf69](https://github.com/agentmail-to/agentmail-cli/commit/d0fbf69778cfba589c91162a01bc111c19d03054))
* improve linking behavior when developing on a branch not in the Go SDK ([7a56630](https://github.com/agentmail-to/agentmail-cli/commit/7a566301e20873025326c753da2fe601e455879b))
* improved workflow for developing on branches ([eb3e18e](https://github.com/agentmail-to/agentmail-cli/commit/eb3e18ec65da5b7b9e43ba20993af7943807be89))
* no longer require an API key when building on production repos ([d82fd7d](https://github.com/agentmail-to/agentmail-cli/commit/d82fd7d528fa133a93bf7d94165382d37fe873df))
* only set client options when the corresponding CLI flag or env var is explicitly set ([6cfe6d4](https://github.com/agentmail-to/agentmail-cli/commit/6cfe6d4f1d49f539dde18233414d3c6132ec5dc9))


### Chores

* **ci:** skip lint on metadata-only changes ([4121a53](https://github.com/agentmail-to/agentmail-cli/commit/4121a539cb2c4bba95a9f54f9be4cdf19322741e))
* **ci:** skip uploading artifacts on stainless-internal branches ([5a4be3e](https://github.com/agentmail-to/agentmail-cli/commit/5a4be3e1c740c2b1f89d116f6cd9348fd81a5dae))
* **internal:** codegen related update ([41d32f3](https://github.com/agentmail-to/agentmail-cli/commit/41d32f3b7bcb7c421d033e0d18c4cf70f7343dd2))
* **internal:** tweak CI branches ([6d81988](https://github.com/agentmail-to/agentmail-cli/commit/6d8198801ce6a24c892d313650ef37cc7e3920b1))
* **internal:** update gitignore ([4532e12](https://github.com/agentmail-to/agentmail-cli/commit/4532e12f8377f26939313361426e9ccd953c4312))
* omit full usage information when missing required CLI parameters ([d2d7873](https://github.com/agentmail-to/agentmail-cli/commit/d2d787381f2c903d50926fbc49d515ff67893e85))

## 0.7.1 (2026-03-06)

Full Changelog: [v0.7.0...v0.7.1](https://github.com/agentmail-to/agentmail-cli/compare/v0.7.0...v0.7.1)

## 0.7.0 (2026-03-06)

Full Changelog: [v0.6.0...v0.7.0](https://github.com/agentmail-to/agentmail-cli/compare/v0.6.0...v0.7.0)

### Features

* **api:** api update ([36258c7](https://github.com/agentmail-to/agentmail-cli/commit/36258c780cc78405beed3e3e40ee53559ccbf75c))

## 0.6.0 (2026-03-05)

Full Changelog: [v0.5.0...v0.6.0](https://github.com/agentmail-to/agentmail-cli/compare/v0.5.0...v0.6.0)

### Features

* **api:** api update ([0ba315c](https://github.com/agentmail-to/agentmail-cli/commit/0ba315c3d27125665f679f7ce210a751606c8ed8))

## 0.5.0 (2026-03-05)

Full Changelog: [v0.4.2...v0.5.0](https://github.com/agentmail-to/agentmail-cli/compare/v0.4.2...v0.5.0)

### Features

* **api:** api update ([3a21b41](https://github.com/agentmail-to/agentmail-cli/commit/3a21b416c3a2374838176c783dac7d8f7afa93d6))


### Bug Fixes

* avoid printing usage errors twice ([e90106c](https://github.com/agentmail-to/agentmail-cli/commit/e90106ccb35b2eacafd86e15bd7385985a2415ef))

## 0.4.2 (2026-03-04)

Full Changelog: [v0.4.1...v0.4.2](https://github.com/agentmail-to/agentmail-cli/compare/v0.4.1...v0.4.2)

## 0.4.1 (2026-03-03)

Full Changelog: [v0.4.0...v0.4.1](https://github.com/agentmail-to/agentmail-cli/compare/v0.4.0...v0.4.1)

## 0.4.0 (2026-03-02)

Full Changelog: [v0.3.0...v0.4.0](https://github.com/agentmail-to/agentmail-cli/compare/v0.3.0...v0.4.0)

### Features

* **api:** api update ([76ce4cd](https://github.com/agentmail-to/agentmail-cli/commit/76ce4cd90b7d7015a942e9f5ab816c46e5ae5042))

## 0.3.0 (2026-03-02)

Full Changelog: [v0.2.0...v0.3.0](https://github.com/agentmail-to/agentmail-cli/compare/v0.2.0...v0.3.0)

### Features

* **api:** api update ([37faa18](https://github.com/agentmail-to/agentmail-cli/commit/37faa1806e930629ed264d1ad84a013384d2c5ff))

## 0.2.0 (2026-03-02)

Full Changelog: [v0.1.0...v0.2.0](https://github.com/agentmail-to/agentmail-cli/compare/v0.1.0...v0.2.0)

### Features

* **api:** api update ([a599bfb](https://github.com/agentmail-to/agentmail-cli/commit/a599bfbc932459617c482162c77bbe218315fc5a))

## 0.1.0 (2026-03-02)

Full Changelog: [v0.0.2...v0.1.0](https://github.com/agentmail-to/agentmail-cli/compare/v0.0.2...v0.1.0)

### Features

* **api:** api update ([51ec860](https://github.com/agentmail-to/agentmail-cli/commit/51ec86042a5139e6a7115c3b61579b5b358748a8))
* **api:** api update ([2199938](https://github.com/agentmail-to/agentmail-cli/commit/2199938961fe622bd008eb779544e35d0dae6368))
* **api:** api update ([ecb9444](https://github.com/agentmail-to/agentmail-cli/commit/ecb94444b5eca11efe328d1fc5bf9084b9bf7ee8))

## 0.0.2 (2026-03-02)

Full Changelog: [v0.0.1...v0.0.2](https://github.com/agentmail-to/agentmail-cli/compare/v0.0.1...v0.0.2)

### Chores

* sync repo ([3fcf10d](https://github.com/agentmail-to/agentmail-cli/commit/3fcf10d0b9c49193616513db312122c3bc98ef8f))
* update SDK settings ([55fe742](https://github.com/agentmail-to/agentmail-cli/commit/55fe74252155d07e0a1c36233626b3919ce14139))
