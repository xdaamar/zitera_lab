import os

files = [
    'labs/A01/README.md',
    'labs/A01/lesson/architecture.id.md',
    'labs/A01/lesson/architecture.md',
    'labs/A01/lesson/concept.md',
    'labs/A01/lesson/walkthrough.md',
    'labs/A02/README.md',
    'labs/A02/lesson/analogy.id.md',
    'labs/A02/lesson/analogy.md',
    'labs/A02/lesson/architecture.id.md',
    'labs/A02/lesson/architecture.md',
    'labs/A02/lesson/walkthrough.id.md',
    'labs/A02/lesson/walkthrough.md'
]

replacements = [
    ('â\x9dŒ', '❌'),
    ('â–²', '▲'),
    ('â–º', '►'),
    ('â–¼', '▼'),
    ('â—„', '◄'),
    ('â”\x90', '┐'),
    ('â”¬', '┬'),
    ('â”Œ', '┌'),
    ('â”œ', '├'),
    ('â”˜', '┘'),
    ('â”‚', '│'),
    ('â””', '└'),
    ('â”€', '─'),
    ('â‰¥', '≥'),
    ('â€”', '—'),
    ('â€¢', '•'),
    ('â†’', '→'),
    ('âœ…', '✅'),
    ('âœ”', '✔'),
]

for file_path in files:
    if not os.path.exists(file_path):
        print(f"Skipping missing file: {file_path}")
        continue
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()

    original = content
    for bad, good in replacements:
        content = content.replace(bad, good)

    if content != original:
        with open(file_path, 'w', encoding='utf-8', newline='\n') as f:
            f.write(content)
        print(f"Repaired: {file_path}")
    else:
        print(f"No changes: {file_path}")

print("Repair completed.")
