#!/bin/bash

cd "$( dirname "${BASH_SOURCE[0]}" )"

simple-http-server -i ./app/ -p 8000 --nocache --try-file ./app/index.html
