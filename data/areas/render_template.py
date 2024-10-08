from jinja2 import Environment, FileSystemLoader
from pathlib import Path
import json


env = Environment(
    loader=FileSystemLoader("."),
)


def main():
    template = env.get_template("insert-areas.sql.j2")
    data = json.loads(Path("areas.json").read_text())
    print(template.render(features=data["features"]))


if __name__ == "__main__":
    main()
