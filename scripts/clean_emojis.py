import os
import re

EMOJI_PATTERN = re.compile(r'[\U00010000-\U0010ffff]|[\u2600-\u27bf]|[\u2300-\u23ff]|[\u2b50-\u2b55]')

TEXT_REPLACEMENTS = [
    # Heading & callout emojis
    ('### 🎯 MISI BRIEFING', '### MISI BRIEFING'),
    ('### 🏢 RUANG LINGKUP & TARGET', '### RUANG LINGKUP & TARGET'),
    ('### 🚩 OBJEKTIF PENELUSURAN', '### OBJEKTIF PENELUSURAN'),
    ('### ⚠️ ATURAN PENGUJIAN (RULES OF ENGAGEMENT)', '### ATURAN PENGUJIAN (RULES OF ENGAGEMENT)'),
    ('### ⚠️ ATURAN PENGUJIAN', '### ATURAN PENGUJIAN'),
    ('### 🔑 FORMAT SUBMISSION', '### FORMAT SUBMISSION'),
    ('### 🔴 1. Kenapa Ini Sangat Berbahaya? (Business & Financial Impact)', '### 1. Kenapa Ini Sangat Berbahaya? (Business & Financial Impact)'),
    ('### 🔴 1. Kenapa Ini Sangat Berbahaya?', '### 1. Kenapa Ini Sangat Berbahaya?'),
    ('### 🔍 2. Bagaimana Cara Mencari & Mengujinya? (Recon & Vulnerability Indicators)', '### 2. Bagaimana Cara Mencari & Mengujinya? (Recon & Vulnerability Indicators)'),
    ('### 🔍 2. Bagaimana Cara Mencari & Mengujinya?', '### 2. Bagaimana Cara Mencari & Mengujinya?'),
    ('### 🛡️ 3. Bagaimana Cara Memperbaikinya? (Secure Coding & Remediation)', '### 3. Bagaimana Cara Memperbaikinya? (Secure Coding & Remediation)'),
    ('### 🛡️ 3. Bagaimana Cara Memperbaikinya?', '### 3. Bagaimana Cara Memperbaikinya?'),
    ('### 🛡 3. Bagaimana Cara Memperbaikinya?', '### 3. Bagaimana Cara Memperbaikinya?'),
    ('### 📝 Petunjuk Teknis:', '### Petunjuk Teknis:'),
    ('### 📝 Petunjuk Teknis', '### Petunjuk Teknis'),
    ('### ⚙️ Bagaimana Cara Kerja Sistem?', '### Bagaimana Cara Kerja Sistem?'),
    ('### ⚙ Bagaimana Cara Kerja Sistem?', '### Bagaimana Cara Kerja Sistem?'),
    ('> **💡 Di Balik Layar: Apa yang terjadi di server?**', '> [!NOTE]\n> **Di Balik Layar: Apa yang terjadi di server?**'),
    ('> **💡 Di Balik Layar:', '> [!NOTE]\n> **Di Balik Layar:'),

    # Table entries
    ('| ❌ |', '| [TIDAK] |'),
    ('| ❌ ', '| [TIDAK] '),
    (' ❌ |', ' [TIDAK] |'),
    ('| ✅ |', '| [DIIZINKAN] |'),
    ('| ✅ ', '| [DIIZINKAN] '),
    (' ✅ |', ' [DIIZINKAN] |'),

    # Generic emojis to professional tags
    ('🎯 ', '[TARGET] '),
    ('🏢 ', '[SCOPE] '),
    ('🚩 ', '[FLAG] '),
    ('⚠️ ', '[PERINGATAN] '),
    ('⚠ ', '[PERINGATAN] '),
    ('🔑 ', '[KEY] '),
    ('🔴 ', '[RISIKO] '),
    ('🔍 ', '[RECON] '),
    ('🛡️ ', '[SECURITY] '),
    ('🛡 ', '[SECURITY] '),
    ('💡 ', '[TIPS] '),
    ('📝 ', '[CATATAN] '),
    ('⚙️ ', '[SISTEM] '),
    ('⚙ ', '[SISTEM] '),
    ('💰 ', '[BOUNTY] '),
    ('⚖️ ', '[REGULASI] '),
    ('⚖ ', '[REGULASI] '),
    ('📋 ', '[CHECKLIST] '),
    ('🎉 ', '[VERIFIED] '),
    ('🔓 ', '[UNLOCKED] '),
    ('🏆 ', '[ACHIEVEMENT] '),
    ('🚨 ', '[ALERT] '),
    ('💎 ', '[PREMIUM] '),
    ('💳 ', '[PAYMENT] '),
    ('🗝️ ', '[KEY] '),
    ('🗝 ', '[KEY] '),
    ('🧠 ', '[MINDSET] '),
    ('💻 ', '[CONSOLE] '),
    ('🏥 ', '[CLINICAL] '),
    ('👤 ', '[USER] '),
    ('📌 ', '[NOTE] '),
    ('📊 ', '[METRICS] '),
    ('🚚 ', '[LOGISTICS] '),
    ('📄 ', '[DOC] '),
    ('📁 ', '[DIR] '),
    ('⚡ ', '[FAST] '),
    ('🚀 ', '[EXEC] '),
    ('❌', '[RENTAN]'),
    ('✅', '[AMAN]'),
]

def clean_text(content: str) -> str:
    for old, new in TEXT_REPLACEMENTS:
        content = content.replace(old, new)
    # Final pass: any lingering emoji stripped
    content = EMOJI_PATTERN.sub('', content)
    return content

def clean_file(filepath: str):
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            orig = f.read()
        cleaned = clean_text(orig)
        if cleaned != orig:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(cleaned)
            return True
    except Exception as e:
        print(f"Error cleaning {filepath}: {e}")
    return False

def run_clean():
    # 1. Clean offlineLabData.ts
    offline_data = 'ui/tauri/src/offlineLabData.ts'
    if os.path.exists(offline_data):
        if clean_file(offline_data):
            print(f"Cleaned {offline_data}")

    # 2. Clean all markdown & json files in labs/
    cleaned_labs = 0
    for root, _, files in os.walk('labs'):
        for f in files:
            if f.endswith(('.md', '.json')):
                fp = os.path.join(root, f)
                if clean_file(fp):
                    cleaned_labs += 1
    print(f"Cleaned {cleaned_labs} files in labs/")

    # 3. Clean engine rust bin files
    cleaned_bins = 0
    for root, _, files in os.walk('engine/rust/src/bin'):
        for f in files:
            if f.endswith('.rs'):
                fp = os.path.join(root, f)
                if clean_file(fp):
                    cleaned_bins += 1
    print(f"Cleaned {cleaned_bins} rust lab binaries in engine/rust/src/bin/")

if __name__ == '__main__':
    run_clean()
