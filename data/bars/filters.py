from typing import (
    Dict,
    List,
    Final,
    Set,
    Iterator,
    Iterable,
    Callable,
    Tuple,
)
import json

from data.bars.models import Feature
from gmaps import get_likeliest_place


MANUAL_ZAAK_NAAM_REPLACEMENTS: Final[Dict[str, str]] = {
    "Café Beurre B.V., Van Limburg Stirumstraat 115": "Café Beurre",
    "Café Bloemers/Colak Holding B.V.": "Café Bloemers",
    "Mooy (eig. B.C. van Baaijen)": "Café Mooy",
    "Mediacentrum de Kroon B.V. (Zn)": "Café Restaurant De Kroon",
    "Hannekes Boom...Sinds 1662": "Hannekes Boom",
    "M.B.M. Horeca (Club Smokey)": "Club Smokey",
    "Gollem Proeflokaal - Gollem D. Stalpertstraat B.V": "Gollem Proeflokaal",
}

MANUAL_EXCLUDED_ZAAK_NUMMER: Final[Set[int]] = {
    10005,  # Lovers Horeca
    12386,  # Dubai Lounge
    18279,  # Smashburgers (actually restaurant)
    17984,  # Cannibale Royale Amstelveenseweg
}

MANUAL_INCLUDED_ZAAK_NUMMER: Final[Set[int]] = {
    15742,  # Clos
}


def apply_filter(
    data: Iterable[Feature], fn: Callable[[Feature], Tuple[bool, Feature]]
) -> Iterator[Feature]:
    for d in data:
        is_ok, new_d = fn(d)
        if is_ok:
            yield new_d


def filter_on_cafes(f: Feature) -> Tuple[bool, Feature]:
    return f.properties.zaak_categorie in {
        "Café",
        "Café met zaalverhuur",
    }, f


def filter_on_nachtzaak(f: Feature) -> Tuple[bool, Feature]:
    return f.properties.zaak_categorie in {"Nachtzaak"}, f


def filter_on_restaurants(f: Feature) -> Tuple[bool, Feature]:
    return f.properties.zaak_categorie == "Restaurant", f


def filter_on_restaurant_name(f: Feature) -> Tuple[bool, Feature]:
    if f.properties.zaaknaam is None:
        return False, f

    name = f.properties.zaaknaam.lower()
    is_cafe = "cafe" in name or "café" in name or "bar" in name
    return is_cafe or f.properties.zaaknummer in MANUAL_INCLUDED_ZAAK_NUMMER, f


def filter_manual_exclusions(f: Feature) -> Tuple[bool, Feature]:
    return f.properties.zaaknummer not in MANUAL_EXCLUDED_ZAAK_NUMMER, f


def filter_coffeeshops(f: Feature) -> Tuple[bool, Feature]:
    if f.properties.zaaknaam is None:
        return False, f
    return "coffee" not in f.properties.zaaknaam.lower(), f


def filter_hotels(f: Feature) -> Tuple[bool, Feature]:
    if f.properties.zaaknaam is None:
        return False, f
    return "hotel" not in f.properties.zaaknaam.lower(), f


def _filter_and_enrich_using_gmaps(
    f: Feature, enforce_bar_type: bool
) -> Tuple[bool, Feature]:
    """
    Enable "enforce_bar_types" for stricter search which yields more actual bars, but also yields
    fewer results than a general address/name search. (Some bars don't show up when searching for
    the "bar" type)
    """
    lng, lat = json.loads(f.geometry)["coordinates"]
    if not f.properties.zaaknaam:
        return False, f

    result = get_likeliest_place(
        f.properties.zaaknaam,
        f.properties.adres,
        (lat, lng),
        enforce_bar_type=enforce_bar_type,
    )

    if result is None:
        return False, f

    # print(f.properties.zaaknaam, " -> ", result["name"])
    f.properties.zaaknaam = result["name"]
    f.properties.google_place_id = result["place_id"]

    return True, f


def filter_and_enrich_using_gmaps(f: Feature) -> Tuple[bool, Feature]:
    """
    Use Google Maps Place API to cross-reference with Gemeente Amsterdam data.
    This generally yields better location names and Google keeps better track of bars that are
    still in business.
    """
    return _filter_and_enrich_using_gmaps(f, False)


def filter_and_enrich_using_gmaps_enforce_bar(f: Feature) -> Tuple[bool, Feature]:
    return _filter_and_enrich_using_gmaps(f, True)


def manual_substitutions_zaaknaam(f: Feature) -> Tuple[bool, Feature]:
    if f.properties.zaaknaam in MANUAL_ZAAK_NAAM_REPLACEMENTS:
        f.properties.zaaknaam = MANUAL_ZAAK_NAAM_REPLACEMENTS[f.properties.zaaknaam]

    return True, f


def beautify_zaaknaam(f: Feature) -> Tuple[bool, Feature]:
    if f.properties.zaaknaam is None:
        return False, f

    f.properties.zaaknaam = (
        f.properties.zaaknaam.replace(" Amsterdam B.V.", "")
        .replace(" B.V.", "")
        .replace(" B.V", "")
        .replace(" VOF", "")
        .replace("V.O.F. ", "")
        .strip()
    )
    return True, f


def prepare_data(data: Iterable[Feature]) -> List[Feature]:
    """Filter and sanitize data"""
    nachtzaak_operations = [
        filter_on_nachtzaak,
        filter_and_enrich_using_gmaps,
    ]

    cafe_operations = [
        filter_on_cafes,
        filter_coffeeshops,
        filter_manual_exclusions,
        manual_substitutions_zaaknaam,
        beautify_zaaknaam,
        filter_and_enrich_using_gmaps,
    ]

    # Some restaurants are also bars, we try to pull some additional bars from there
    restaurant_operations = [
        filter_on_restaurants,
        filter_on_restaurant_name,
        beautify_zaaknaam,
        filter_and_enrich_using_gmaps_enforce_bar,
    ]

    cafe_data: Iterable[Feature] = data
    for operation in cafe_operations:
        cafe_data = apply_filter(cafe_data, operation)

    restaurant_data: Iterable[Feature] = data
    for operation in restaurant_operations:
        restaurant_data = apply_filter(restaurant_data, operation)

    nachtzaak_data: Iterable[Feature] = data
    for operation in nachtzaak_operations:
        nachtzaak_data = apply_filter(nachtzaak_data, operation)

    return list(nachtzaak_data) + list(cafe_data) + list(restaurant_data)
