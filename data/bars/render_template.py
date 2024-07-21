from jinja2 import Environment, FileSystemLoader
from pathlib import Path
import json


env = Environment(
    loader=FileSystemLoader("."),
)


def main():
    template = env.get_template("insert-bars.sql.j2")
    data = json.loads(Path("cafes.json").read_text())
    print(template.render(locations=data))


if __name__ == "__main__":
    main()
