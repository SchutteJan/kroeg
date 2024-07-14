# Areas

Areas define geographic regions in which locations can lie, the municipality of Amsterdam defines 6 kinds of areas:

- Buurt (neighborhood), i.e. Dapperbuurt-Zuid
- Wijk (district), i.e. Oosterparkbuurt
- Gebied (area), i.e. Watergraafsmeer
- Stadsdeel/stadsgebied (borough), i.e. West
- MRA Gemeenten, i.e. Amstelveen
- Vervoerregio, i.e. Amsterdam

Source: https://maps.amsterdam.nl/gebiedsindeling/


## Data Import

https://maps.amsterdam.nl/open_geodata/

```bash
cd data/areas
curl 'https://maps.amsterdam.nl/open_geodata/geojson_lnglat.php?KAARTLAAG=INDELING_GEBIED&THEMA=gebiedsindeling' > areas.json

# Inspect all area names:
jq ".features[].properties.Gebied" areas.json

# Generate SQL import data for areas table
poerty run python render_template.py
```


## Example queries

```postgresql
SELECT an_location.name, ST_Contains(an_area.area, an_location.coordinates) As areacontainslocation
FROM
    (SELECT area FROM areas WHERE name = 'Oud-Oost') As an_area,
    (SELECT coordinates, name FROM locations) As an_location;
```
