import json


with open("discovery.json") as f:
    dat = json.load(f)

print(dat)

for entry in dat["lines"]:
    entry["sid"] = entry["speaker"][0].upper()

with open("discovery.json", 'w') as f:
    json.dump(dat, f, ensure_ascii=False)
