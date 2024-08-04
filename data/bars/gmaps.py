import math
from typing import Tuple, Optional, Dict

import requests
from diskcache import Cache

import googlemaps
import os

cache = Cache("./gmaps_cache")


def get_gmaps_client() -> googlemaps.Client:
    api_key = os.environ.get("GMAPS_API")
    if not api_key:
        raise ValueError("No Google Maps API key found")
    return googlemaps.Client(key=api_key)


# returns distance in meters between two lat/long points
def get_distance(a: Tuple[float, float], b: Tuple[float, float]) -> float:
    # Radius of the Earth in meters
    R = 6371000

    # Convert latitude and longitude from degrees to radians
    lat1, lon1 = math.radians(a[0]), math.radians(a[1])
    lat2, lon2 = math.radians(b[0]), math.radians(b[1])

    # Differences in coordinates
    dlat = lat2 - lat1
    dlon = lon2 - lon1

    # Haversine formula
    h = (
        math.sin(dlat / 2) ** 2
        + math.cos(lat1) * math.cos(lat2) * math.sin(dlon / 2) ** 2
    )
    c = 2 * math.atan2(math.sqrt(h), math.sqrt(1 - h))

    # Distance in meters
    distance = R * c

    return distance


@cache.memoize(typed=True)
def gmaps_place_details(place_id: str) -> dict:
    gmaps = get_gmaps_client()
    return gmaps.place(place_id)


@cache.memoize(typed=True)
def gmaps_place_search(
    address: str,
    location: Tuple[float, float],
    type: Optional[str],
    language: Optional[str] = None,
) -> dict:
    gmaps = get_gmaps_client()
    return gmaps.places(address, location=location, language=language, type=type)


@cache.memoize(typed=True)
def get_url_redirect(url: str) -> str:
    r = requests.get(url, headers={"Range": "bytes=0-0"}, allow_redirects=False)
    r.raise_for_status()
    return r.headers["Location"]


def get_image_url(place_id: str) -> Optional[str]:
    try:
        place_details = gmaps_place_details(place_id)
        photo_reference = place_details["result"]["photos"][0]["photo_reference"]
    except Exception as e:
        print(f"No photos found for {place_id}: {e}")
        return None

    # Don't want to expose the Google Maps API key, so instead we get the redirected image URL instead
    gmaps_url = f"https://maps.googleapis.com/maps/api/place/photo?maxwidth=512&photo_reference={photo_reference}&key={os.environ.get("GMAPS_API")}"
    return get_url_redirect(gmaps_url)


def get_likeliest_place(
    expected_name: str,
    address: str,
    location: Tuple[float, float],
    enforce_bar_type: bool = False,
) -> Optional[Dict]:
    # Replace - with a space, as Google doesn't format addresses with dashes and it seems to improve search results
    address = address.replace("-", " ")

    # Not passing a type parameter can result in better search results, we will check for it later
    results = gmaps_place_search(
        f"{expected_name}, {address}, Amsterdam",
        location=location,
        type=None,
        language="nl",
    )

    if len(results["results"]) == 0:
        print(f"No results found for {expected_name} on {address}")
        return None

    # We assume the first result is the likeliest match
    result = results["results"][0]

    # Check the distance isn't too big
    distance = get_distance(
        location,
        (result["geometry"]["location"]["lat"], result["geometry"]["location"]["lng"]),
    )

    # This seems large, but there can be quite a discrepancy in the location of the bar as known by
    # Gemeente Amsterdam and Google Maps. Example: 't Blauwe Theehuis
    if distance > 300:
        print(
            f"Distance between '{expected_name}' and '{result['name']}' is too large ({distance}) meters"
        )
        return None

    # result has any of the ALLOWED_TYPES as type
    allowed_types = (
        ["bar", "cafe", "restaurant", "establishment"]
        if not enforce_bar_type
        else ["bar", "cafe"]
    )
    if not any(t in result["types"] for t in allowed_types):
        print(
            f"Result for '{expected_name}' is '{result['name']}' and is not a bar/cafe, but a {result['types']}"
        )
        return None

    if result["business_status"] != "OPERATIONAL":
        print(
            f"Result for '{expected_name}' is '{result['name']}' but is not in business"
        )
        return None

    # print(json.dumps(results, indent=2))
    return result


if __name__ == "__main__":
    # 't Blauwe theehuis has slightly different name according to Google, and has a slightly different geolocation
    # get_likeliest_place("'t Blauwe Theehuis", "Vondelpark 5 A", location=(52.35835998002426, 4.870637924609814))

    # A location that should not be in our results because it doesn't have the bar type on Google Maps
    # get_likeliest_place("Koffiehuis De Hoek", "Prinsengracht 341-H",
    #                     location=(52.372331919453636, 4.883287629621083))

    # "Amigo Cafe" which doesn't exist on this address according to google. There is a cafe called
    # "Brakke grond" however that has been closed recently.
    # get_likeliest_place("Amigo Cafe", "Rozengracht 16", location=(52.373917975661, 4.8821682457514))

    # Vondelpark 3 has been renamed to "Park Zuid"
    # get_likeliest_place("Vondelpark 3", "Vondelpark 3", location=(52.36105259638123, 4.874974382779035))

    # Café Oost is a bar, but is listed as "cafe" on Google Maps (does not show up in type="something" results that
    # are not exact matches)
    # get_likeliest_place("Café Oost", "Krugerplein 40", location=(52.3534727, 4.9198154))

    # Arie Goudvisch is definitely a bar, but because the address has a -, Google can't find it :S
    # get_likeliest_place("Arie Goudvisch", "Ferdinand Bolstraat 24-H", location=(52.356687726284804, 4.890481919493629))
    pass
