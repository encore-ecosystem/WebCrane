from pathlib import Path

def save_default_dotignore(webcrane_path: Path):
    with open(webcrane_path / '.webcraneignore', 'w') as f:
        f.write(webcrane_path.__str__())

__all__ = ['save_default_dotignore']