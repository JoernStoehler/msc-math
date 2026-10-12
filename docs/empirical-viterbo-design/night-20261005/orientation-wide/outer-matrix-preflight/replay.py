#!/usr/bin/env python3
"""Read-only exact replay of cone.json; no file writes."""
from generate import build,ROOT
import json

def math_payload(packet):
    return {key:value for key,value in packet.items() if key!='elapsed_seconds'}

def main():
    retained=json.loads((ROOT/'cone.json').read_text());fresh=build()
    assert math_payload(retained)==math_payload(fresh)
    print('PASS read-only exact cone inventory, basis/gauge, strict witnesses, reflection and source/control replay')

if __name__=='__main__':main()
