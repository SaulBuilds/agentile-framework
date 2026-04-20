# music-box-sdk

Python SDK for the [state-space-music-box](https://github.com/SaulBuilds/agentile-framework) HTTP API.

## Install

```bash
pip install music-box-sdk
```

## Quick Start

```python
from music_box_sdk import MusicBoxClient

client = MusicBoxClient("http://localhost:3001", "your-api-key")

# Explore: sweep 10 seeds
sweep = client.parameter_sweep("demo", seeds=[1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
print("Best seed:", sweep["ranked_seeds"][0])

# Create a session with the best seed
session = client.create_session("My Session", "demo", seed=sweep["ranked_seeds"][0])

# Render a preview
preview = client.render_preview(session["session_id"])

# Adapt: patch the preset
client.preset_patch("demo", reason="faster tempo", tempo_bpm=140, low_note=48)
```

## API Reference

See [docs/AGENT_GUIDE.md](https://github.com/SaulBuilds/agentile-framework/blob/main/docs/AGENT_GUIDE.md) for the complete tool reference.

## License

MIT OR Apache-2.0
