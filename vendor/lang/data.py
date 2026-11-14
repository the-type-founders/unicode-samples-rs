# /// script
# requires-python = ">=3.9"
# dependencies = ["gfmetadata"]
# ///
import json
import os
import sys

ROOT = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(ROOT, "source", "Lib"))

import gflanguages

languages = gflanguages.LoadLanguages(os.path.join(ROOT, "source", "Lib", "gflanguages", "data"))
data = []
for _, language in sorted(languages.items()):
    sample = language.sample_text
    if not sample.masthead_partial and not sample.tester:
        continue
    data.append(
        {
            "language": language.language,
            "script": language.script,
            "population": language.population if language.HasField("population") else None,
            "grid": sample.masthead_partial or None,
            "line": sample.tester or None,
        }
    )
json.dump(data, sys.stdout, ensure_ascii=False, indent=2)
sys.stdout.write("\n")
