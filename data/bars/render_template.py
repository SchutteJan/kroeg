from jinja2 import Environment, FileSystemLoader
from pathlib import Path

from data.bars.datasets import load_gemeente_amsterdam_dataset
from data.bars.filters import prepare_data

env = Environment(
    loader=FileSystemLoader("."),
)


def main() -> None:
    template = env.get_template("insert-bars.sql.j2")
    dataset = load_gemeente_amsterdam_dataset()
    print(f"Data points before filtering: {len(dataset)}")
    prepared_data = prepare_data(dataset)
    print(f"Data points after filtering: {len(prepared_data)}")
    sql = template.render(features=prepared_data)

    print("Wrote to bars-export.sql")
    Path("bars-export.sql").write_text(sql)


if __name__ == "__main__":
    main()
