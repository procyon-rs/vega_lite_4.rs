import sys
import re

changes = [
    {
        'struct': 'NormalizedSpecSpec',
        'from': r'pub spec: Box<Option<NormalizedSpecSpec>>',
        'to': r'pub spec: Option<Box<NormalizedSpecSpec>>'
    },
    {
        'struct': 'Stream',
        'from': r'pub stream: Box<Option<Stream>>',
        'to': r'pub stream: Option<Box<Stream>>'
    },
    {
        'struct': 'ViewConfig',
        'from': r'pub stroke: Option<String>',
        'to': r'#[serde(default, skip_serializing_if = "RemovableValue::is_default")] #[builder(default)] pub stroke: RemovableValue<String>'
    },
    {
        'struct': 'Encoding',
        'from': r'pub tooltip: Option<Tooltip>',
        'to': r'#[serde(default, skip_serializing_if = "RemovableValue::is_default")] #[builder(default)] pub tooltip: RemovableValue<Tooltip>'
    },
    {
        'struct': 'BaseMarkConfig',
        'from': r'pub tooltip: Option<serde_json::Value>',
        'to': r'#[serde(default, skip_serializing_if = "RemovableValue::is_default")] #[builder(default)] pub tooltip: RemovableValue<serde_json::Value>'
    },
    {
        'struct': 'LayerEncoding',
        'from': r'pub tooltip: Option<Tooltip>',
        'to': r'#[serde(default, skip_serializing_if = "RemovableValue::is_default")] #[builder(default)] pub tooltip: RemovableValue<Tooltip>'
    },
    # Boxing of the following 3 fields to avoid the following error
    #   test tests::serde_should_not_failed_on_empty ... 
    #   thread 'main' has overflowed its stack
    #   fatal runtime error: stack overflow
    {
        'struct': 'Vegalite',
        'from': r'pub config: Option<Config>',
        'to': r'pub config: Option<Box<Config>>'
    },
    {
        'struct': 'Vegalite',
        'from': r'pub encoding: Option<EdEncoding>',
        'to': r'pub encoding: Option<Box<EdEncoding>>'
    },
    {
        'struct': 'Vegalite',
        'from': r'pub spec: Option<VegaliteSpec>',
        'to': r'pub spec: Option<Box<VegaliteSpec>>'
    },
]

change_is_done = {}

with open(sys.argv[1], 'r', encoding="utf-8", errors='replace') as f:
    with open(sys.argv[2], 'w+', encoding="utf-8", errors='replace') as t:
        for line in f:
            rewrote = False
            for (change_id, change) in enumerate(changes):
                if "struct " + change["struct"] + " {" in line:
                    change_is_done[change_id] = False
                if change_id in change_is_done and not change_is_done[change_id] and re.search(change["from"], line):
                    print("rewrote " + change["struct"] + " field")
                    t.write(re.sub(change["from"], change["to"], line))
                    change_is_done[change_id] = True
                    rewrote = True
            if not rewrote:
                t.write(line)
