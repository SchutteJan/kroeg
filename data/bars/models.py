import json
from dataclasses import dataclass
from datetime import datetime
from typing import Optional, Dict, Any

GeoJsonStr = str


@dataclass
class Properties:
    id: int
    zaaknummer: int
    zaaknaam: Optional[str]  # Why this is optional is beyond me
    adres: str
    zaak_categorie: str
    zaak_specificatie: str
    begindatum: str
    einddatum: str
    openingstijden_zo_do_van: str
    openingstijden_zo_do_tot: str
    openingstijden_vr_za_van: str
    openingstijden_vr_za_tot: str
    o_tijden_terras_zo_do_van: Optional[str]
    o_tijden_terras_zo_do_tot: Optional[str]
    o_tijden_terras_vr_za_van: Optional[str]
    o_tijden_terras_vr_za_tot: Optional[str]
    postcode: str
    status_vergunning: str
    status_tijdelijk_terras: Optional[str]
    toestemming_tijdelijk_terras: Optional[str]
    publ_besluit_tijdelijk_terras: Optional[str]
    tijdelijk_terras_details: Optional[str]
    status_verlenging_tijdelijk_terras: Optional[str]
    verlenging_tijdelijk_terras_details: Optional[str]

    # Additional properties added by enriching with Google Maps
    google_place_id: Optional[str] = None

    def format_sql_google_place_id(self) -> str:
        if self.google_place_id:
            return f"'{self.google_place_id}'"
        return "NULL"

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "Properties":
        return cls(**data)


@dataclass
class Feature:
    type: str
    geometry: GeoJsonStr
    properties: Properties

    @classmethod
    def from_json(cls, data: Dict[str, Any]) -> "Feature":
        return cls(
            type=data["type"],
            geometry=json.dumps(data["geometry"]),
            properties=Properties.from_dict(data["properties"]),
        )


# TODO: generate from jsonschema
@dataclass
class LocationResponse:
    id: int
    name: str
    description: Optional[str]
    google_place_id: Optional[str]
    coordinates: Dict[str, float]
    published: bool
    imageurl: Optional[str]
    address_line: str
    visited_at: Optional[datetime]
    area_name: Optional[str]
    gem_ams_id: Optional[int]

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "LocationResponse":
        return cls(
            id=data["id"],
            name=data["name"],
            description=data.get("description"),
            google_place_id=data.get("google_place_id"),
            coordinates=data["coordinates"],
            published=data["published"],
            imageurl=data.get("imageurl"),
            address_line=data["address_line"],
            visited_at=data.get("visited_at"),
            area_name=data.get("area_name"),
            gem_ams_id=data.get("gem_ams_id"),
        )


# TODO: Generate from jsonschema


# TODO: Generate from jsonschema
@dataclass
class NewLocation:
    name: str
    coordinates: Dict[str, float]
    published: bool
    description: Optional[str]
    osm_node_id: Optional[str]
    google_place_id: Optional[str]
    imageurl: Optional[str]
    address_line: Optional[str]
    gem_ams_id: Optional[int]

    def to_dict(self) -> Dict[str, Any]:
        return {
            "name": self.name,
            "coordinates": self.coordinates,
            "published": self.published,
            "description": self.description,
            "osm_node_id": self.osm_node_id,
            "google_place_id": self.google_place_id,
            "imageurl": self.imageurl,
            "address_line": self.address_line,
            "gem_ams_id": self.gem_ams_id,
        }

    @classmethod
    def from_feature(cls, f: Feature):
        coords = json.loads(f.geometry)["coordinates"]
        return cls(
            name=str(f.properties.zaaknaam),
            coordinates={"x": coords[1], "y": coords[0]},
            published=True,
            description=f.properties.zaak_categorie,
            osm_node_id=None,
            google_place_id=f.properties.google_place_id,
            imageurl=None,
            address_line=f.properties.adres,
            gem_ams_id=f.properties.id,
        )

    def __repr__(self) -> str:
        return f"{self.name} - {self.address_line} ({self.gem_ams_id})"
