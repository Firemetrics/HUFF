import yaml
from datetime import datetime
    
# helper function
# returns the value of a key in a dictionary, or None if the key is not present
def or_none(d: dict, k: str) -> any:
    if k in d:
        return d[k]
    else:
        return None

# helper function
# returns True if two arrays are equal disregarding the order of elements, False otherwise
def arr_equal(arr1, arr2):
    size1 = len(arr1)
    size2 = len(arr2)
    if (size1 != size2):
        return False
    arr1.sort()
    arr2.sort()
    for i in range(0, size2):
        if (arr1[i] != arr2[i]):
            return False
    return True

def hf_summarize(o: dict, k:str|None = None) -> str|dict:

    # special case: Reference
    if isinstance(o, dict) and list(o.keys()) == ['reference']:
        return f"Reference({o['reference']})"

    # special case: HumanName
    if k == 'name':
        return hf_name(o)
    
    # special case: telecom
    if k == 'telecom':
        return hf_telecom(o)

    # generic handler
    tags = ['system', 'value', 'unit', 'code', 'version', 'display', 'url', 'valueInstant', 'valueString', 'valueBoolean', 'valueCode']
    summary =  ' | '.join([hf_simplify(o[tag]) for tag in tags if tag in o])        
    if summary:
        return summary
    else:
        return {k: reformat_fhir(v, k) for k,v in o.items()}

def hf_datetime(datetime_str: str) -> str:
    try:
        # Parse the datetime string
        datetime_obj = datetime.strptime(datetime_str, "%Y-%m-%dT%H:%M:%S.%f%z")
        # Format the datetime object into a more human-readable string
        return datetime_obj.strftime("%d.%m.%Y %H:%M:%S %Z")
    except:
        try:
            # Parse the datetime string
            datetime_obj = datetime.strptime(datetime_str, "%Y-%m-%d")
            # Format the datetime object into a more human-readable string
            return datetime_obj.strftime("%d.%m.%Y")
        except:
            return datetime_str
  
def hf_name(o: dict) -> str|dict:
    if 'text' in o:
        return o['text']
    
    given = ''
    if 'given' in o:
        given = ' '.join(o['given'])

    family = or_none(o, 'family')
    prefix = or_none(o, 'prefix')
    suffix = or_none(o, 'suffix')
    use = or_none(o, 'use')

    text = ' '.join([item for item in [prefix, given, family, suffix] if item is not None])

    if use is not None:
        return f"{text} | {use}"
    else:
        return text

def hf_telecom(o: dict) -> str:
    if arr_equal(list(o.keys()), ['system', 'value', 'use']):
        return f"{o['value']} | {o['system']} | {o['use']}"
    if arr_equal(list(o.keys()), ['system', 'value']):
        return f"{o['value']} | {o['system']}"
    return reformat_fhir(o)

def hf_simplify(o: any) -> str:
    if isinstance(o, str):
        o = hf_datetime(o)
        if not o.startswith('|') and ("\n" in o or "\r" in o):
            o = '|\n' + o
    return str(o)

def reformat_fhir(v, k:str|None = None) -> any:

    if k is None:
        if isinstance(v, dict):
            return {k: reformat_fhir(v, k) for k,v in v.items()}
        else:
            raise ValueError(f"Expected dict, got {type(v)}")

    if isinstance(v, dict):
        return hf_summarize(v, k)
    elif isinstance(v, list):
        elements = [reformat_fhir(v2, k) for v2 in v]
        if len(elements) == 1:
            return elements[0]
        return elements
    return hf_simplify(v)

def to_yaml(obj: dict) -> str:
    return yaml.dump(obj, default_flow_style=False, sort_keys=False, width=1000, allow_unicode=True, indent=2)

def friendly(fhir_obj: dict) -> str:
    return to_yaml(reformat_fhir(fhir_obj))