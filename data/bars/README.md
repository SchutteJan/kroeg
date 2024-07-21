# Bars

Verleende exploitatievergunningen horeca met terrasgrenzen en ontheffingen
https://data.amsterdam.nl/data/datasets/GsY50tEkoJKCGw/verleende-exploitatievergunningen-horeca-met-terrasgrenzen-en-ontheffingen?term=Verleende+exploitatievergunningen+horeca+met+terrasgrenzen+en+ontheffingen

https://api.data.amsterdam.nl/dcatd/datasets/GsY50tEkoJKCGw



## Explore

```bash
curl 'https://api.data.amsterdam.nl/dcatd/datasets/GsY50tEkoJKCGw' > bars_metadata.json

# Get the endpoint of the correct data endpoint:
cat bars_metadata.json | jq '."dcat:distribution"[] | select( ."dct:title" == "Exploitatievergunningen")'

# Download dataset from "wfs" type:
curl 'https://api.data.amsterdam.nl/v1/wfs/horeca/?REQUEST=GetFeature&SERVICE=WFS&version=2.0.0&count=5000&typenames=exploitatievergunning&BBOX=4.58565,52.03560,5.31360,52.48769,urn:ogc:def:crs:EPSG::4326&outputformat=geojson&srsName=urn:ogc:def:crs:EPSG::4326' > bars.json

# View location names
jq '.features[].properties.zaaknaam' bars.json

# All types of locations
jq '.features[].properties.zaak_categorie' bars.json | sort -u

# Locations of a specific type
jq '.features[] | select(.properties.zaak_categorie == "Café") | .properties.zaaknaam' bars.json

# Some "zaak_categorie" have sub-categories "zaak_specificatie"
# Example: Cafe -> (Cafe, Eetcafe)
jq '.features[] | select(.properties.zaak_categorie == "Onbekend") | .properties.zaak_specificatie' bars.json
```

## Import

```bash
# Filter on Cafes
jq '.features[] | select(.properties.zaak_categorie == "Café")' bars.json > cafes.json

poetry run python render_template.py

```
