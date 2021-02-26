import glob

GLOB_PATTERNS = ["**/**/*.py", "**/*.py"]
BLACKLIST_PATTERNS = ["target/"]

flattened = []
for pattern in GLOB_PATTERNS:
    for filepath in glob.glob(pattern):
        for blacklist in BLACKLIST_PATTERNS:
            if blacklist not in filepath:
                flattened.append(filepath)

print(str.join(" ", flattened))
