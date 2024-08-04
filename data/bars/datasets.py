from dataclasses import dataclass
from enum import Enum
from typing import List, Dict, Any

import requests
from requests import Session

from data.bars.models import LocationResponse, Feature, NewLocation

KROEG_ENDPOINT_LIVE = "https://kroeg.jan.tf"
KROEG_ENDPOINT_LOCAL = "http://localhost:8080"
AMS_ENDPOINT = "https://api.data.amsterdam.nl/v1/wfs/horeca/?REQUEST=GetFeature&SERVICE=WFS&version=2.0.0&count=5000&typenames=exploitatievergunning&BBOX=4.58565,52.03560,5.31360,52.48769,urn:ogc:def:crs:EPSG::4326&outputformat=geojson&srsName=urn:ogc:def:crs:EPSG::4326"


class Env(Enum):
    LOCAL = "local"
    LIVE = "live"


@dataclass
class Credentials:
    email: str
    password: str

    def to_dict(self) -> dict:
        return {"email": self.email, "password": self.password}


def kroeg_endpoint(env: Env) -> str:
    return KROEG_ENDPOINT_LOCAL if env == Env.LOCAL else KROEG_ENDPOINT_LIVE


def kroeg_credentials(env: Env) -> Credentials:
    if env == Env.LOCAL:
        return Credentials(email="some.user@gmail.com", password="somepassw0rdthatisok")
    else:
        email = input("Email: ")
        password = input("Password: ")
        return Credentials(email=email, password=password)


def load_remote_kroegen_dataset(
    session: Session, env: Env, only_published: bool = False
) -> List[LocationResponse]:
    r = session.get(
        kroeg_endpoint(env) + "/bars?only_published=" + str(only_published).lower()
    )
    r.raise_for_status()
    return [LocationResponse.from_dict(x) for x in r.json()]


def load_gemeente_amsterdam_dataset() -> List[Feature]:
    r = requests.get(AMS_ENDPOINT)
    r.raise_for_status()
    return [Feature.from_json(x) for x in r.json()["features"]]


def authenticate_api(session: Session, env: Env) -> None:
    if "user_id" in session.cookies:
        return

    credentials = kroeg_credentials(env)
    r = session.post(kroeg_endpoint(env) + "/session/login", data=credentials.to_dict())
    r.raise_for_status()


def add_location(f: NewLocation, session: Session, env: Env) -> None:
    authenticate_api(session, env)
    r = session.post(kroeg_endpoint(env) + "/bar", json=f.to_dict())
    r.raise_for_status()


def update_properties_bar(
    bar_id: int, session: Session, env: Env, properties: Dict[str, Any]
) -> None:
    authenticate_api(session, env)
    r = session.patch(kroeg_endpoint(env) + f"/bar/{bar_id}", json=properties)
    r.raise_for_status()
