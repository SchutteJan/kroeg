from typing import Iterable, Dict

import click
import requests

from data.bars.datasets import (
    load_remote_kroegen_dataset,
    load_gemeente_amsterdam_dataset,
    add_location,
    Env,
)
from data.bars import filters
from data.bars.models import Feature, LocationResponse, NewLocation


def id_to_feature(data: Iterable[Feature]) -> Dict[int, Feature]:
    return {f.properties.id: f for f in data}


def id_to_location(
    locations: Iterable[LocationResponse],
) -> Dict[int, LocationResponse]:
    return {loc.gem_ams_id: loc for loc in locations if loc.gem_ams_id is not None}


@click.command()
@click.option("--apply", is_flag=True)
@click.option(
    "--env", type=click.Choice([e.value for e in Env]), default=Env.LOCAL.value
)
def main(apply: bool, env: str):
    e = Env(env)
    if not apply:
        print("DRY RUN - NO DB CHANGES")

    print(f"Operating on {e} environment")

    session = requests.session()

    current_bars = load_remote_kroegen_dataset(session, e)
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
                add_location(new_loc, session, e)
    else:
        print("No new bars")


if __name__ == "__main__":
    main()
