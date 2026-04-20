# @saulbuilds/music-box-sdk

TypeScript SDK for the [state-space-music-box](https://github.com/SaulBuilds/agentile-framework) HTTP API.

## Install

```bash
npm install @saulbuilds/music-box-sdk
```

## Quick Start

```ts
import { MusicBoxClient } from "@saulbuilds/music-box-sdk";

const client = new MusicBoxClient("http://localhost:3001", "your-api-key");

// Explore: sweep 10 seeds and find the most dynamic
const sweep = await client.parameterSweep("demo", [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
console.log("Best seed:", sweep.ranked_seeds[0]);

// Create: start a session with the best seed
const session = await client.createSession("My Session", "demo", sweep.ranked_seeds[0]);

// Render: generate a preview
const preview = await client.renderPreview(session.session_id);

// Adapt: patch the preset and re-sweep
await client.presetPatch("demo", { tempo_bpm: 140, low_note: 48 }, "faster tempo");
const newSweep = await client.parameterSweep("demo", [1, 2, 3, 4, 5]);
```

## API Reference

See [docs/AGENT_GUIDE.md](https://github.com/SaulBuilds/agentile-framework/blob/main/docs/AGENT_GUIDE.md) for the complete tool reference.

## License

MIT OR Apache-2.0
