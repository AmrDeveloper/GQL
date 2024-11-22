import os
import sys
import json
from typing import Dict

def main() -> None:
    """
    Main function to generate a JSON object containing version information and checksums.

    This function reads the version and base directory from command line arguments,
    iterates through the directories and files in the base directory, reads the SHA-256
    checksum files, and prints a JSON object with the version and checksums.

    Command line arguments:
    version (str): The version string.
    base (str): The base directory containing the checksum files.

    Returns:
    None
    """
    version: str = sys.argv[1]
    base: str = sys.argv[2]

    checksums: Dict[str, str] = {}

    for folder in os.listdir(base):
        for filename in os.listdir(os.path.join(base, folder)):
            if filename.endswith(".sha256"):
                with open(os.path.join(base, folder, filename)) as f:
                    sha256: str = f.read().strip()
                checksums[filename[:-7]] = sha256

    print(json.dumps({
        "version": version,
        "checksums": checksums,
    }, indent=2))

if __name__ == "__main__":
    main()
