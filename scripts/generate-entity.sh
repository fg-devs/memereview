#!/usr/bin/env sh

sea-orm-cli generate entity -o src/entity
mv src/entity/mod.rs src/entity.rs
