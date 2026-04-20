"""
music-box-sdk: Python SDK for the state-space-music-box HTTP API.

Usage:
    from music_box_sdk import MusicBoxClient

    client = MusicBoxClient("http://localhost:3001", "your-api-key")
    presets = client.list_presets()
    sweep = client.parameter_sweep("demo", seeds=[1, 2, 3, 4, 5])
    print("Best seed:", sweep["ranked_seeds"][0])
"""

from .client import MusicBoxClient

__version__ = "0.2.0b1"
__all__ = ["MusicBoxClient"]
