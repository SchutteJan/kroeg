#!/bin/bash

set -e

poetry -C data install
poetry -C data run mypy --strict "$@"
