#!/usr/bin/env python3

from hff import friendly
import os, sys, requests

# read an url from args
url = sys.argv[1]

#url = 'https://ship.ume.de/app/FHIR/r4/ImagingStudy/fffffc175b111b1d706f4bda688caa895e387634f41e22137532a206fd74dc0b'
#url = 'https://ship.ume.de/app/FHIR/r4/Patient/ffffca34f3b009c91edf24a4ea88e89863be9e16ca3823fb34f697ebfcf16ab6'
#url = 'https://ship.ume.de/app/FHIR/r4/Observation/fffffff82d7880816dfde04d7af6c7bf786e0c49c61769d7d191f3146bcb9ba1'
#url = 'https://ship.ume.de/app/FHIR/r4/Observation/ffffffec5646d5da82a36df06a8ef2e2a3d705c15ff56debc79a9db9839308f6'
#url = 'https://ship.ume.de/app/FHIR/r4/Observation/000000b2c421a8b365fcdf0156ea4563e1f0da1a11e56ddb19b61628af666366'
#url = 'https://ship.ume.de/app/FHIR/r4/Observation/bf39db05fa38ab4a4e32f10f44c6f8537a9042fe238c179400046e858d4c6afd'

# get token from ENV VAR
token = os.environ['AUTH_TOKEN']
headers = {
    'Authorization': f"Bearer {token}",
}

response = requests.get(url, headers=headers)
fobj = response.json()            
print(friendly(fobj))