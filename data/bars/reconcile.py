from typing import Iterable, Dict

import click
import requests
from requests import Session

from data.bars.datasets import (
    load_remote_kroegen_dataset,
    load_gemeente_amsterdam_dataset,
    add_location,
    Env,
    update_properties_bar,
)
from data.bars import filters
from data.bars.gmaps import get_image_url
from data.bars.models import Feature, LocationResponse, NewLocation


def id_to_feature(data: Iterable[Feature]) -> Dict[int, Feature]:
    return {f.properties.id: f for f in data}


def id_to_location(
    locations: Iterable[LocationResponse],
) -> Dict[int, LocationResponse]:
    return {loc.gem_ams_id: loc for loc in locations if loc.gem_ams_id is not None}


def process_new_bars(apply: bool, env: Env, session: Session):
    current_bars = load_remote_kroegen_dataset(session, env)
    gem_ams_bars = filters.prepare_data(load_gemeente_amsterdam_dataset())

    current_id_to_bar = id_to_location(current_bars)
    ams_id_to_bar = id_to_feature(gem_ams_bars)

    new_bar_ids = set(ams_id_to_bar.keys()) - set(current_id_to_bar.keys())
    if len(new_bar_ids) > 0:
        print("New bars:")
        for new_id in new_bar_ids:
            new_loc = NewLocation.from_feature(ams_id_to_bar[new_id])
            print(new_loc)
            if apply:
                add_location(new_loc, session, env)
    else:
        print("No new bars")


def update_images(apply: bool, env: Env, session: Session, only_missing: bool = True):
    current_bars = load_remote_kroegen_dataset(session, env, only_published=False)

    for bar in current_bars:
        if bar.imageurl is not None and only_missing:
            continue

        if bar.google_place_id is None:
            continue

        image_url = get_image_url(bar.google_place_id)

        if image_url is not None:
            print(f"Updating image for {bar.name}: {image_url}")
            if apply:
                update_properties_bar(bar.id, session, env, {"imageurl": image_url})
        else:
            print(f"No image found for {bar.name}")


@click.command()
@click.option("--apply", is_flag=True)
@click.option(
    "--env", type=click.Choice([e.value for e in Env]), default=Env.LOCAL.value
)
@click.option(
    "--operation", type=click.Choice(["new_bars", "update_images"]), default="new_bars"
)
def main(apply: bool, env: str, operation: str):
    e = Env(env)
    if not apply:
        print("DRY RUN - NO DB CHANGES")

    print(f"Performing {operation} on {e} environment")

    session = requests.session()

    if operation == "new_bars":
        process_new_bars(apply, e, session)

    elif operation == "update_images":
        update_images(apply, e, session)


if __name__ == "__main__":
    main()
