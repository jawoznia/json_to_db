# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/jawoznia/json_to_db/releases/tag/v0.1.0) - 2023-06-29

### Other
- add release.yml
- Added get all method.
- Increased coverage of library interface.
- Removed unneccessary func.
- added try_load_file_with_corrupted_json
- test try_load_non_exisiting_file
- Fixing Json_loader error handling.
- Removed unneccessary pub modifier.
- Moved DbManager to mod.rs as there was no reason for another layer.
- Error handling in case of wrong db path done.
- Do not create new db if one already exists.
- Select by category done with tests.
- Impl and tested since year.
- Query returns empty vec tested.
- Query for laureates from year works as expected.
- Silencing unused and dead_code warnings.
- Implementing query for laureates from given year.
- Added new to Laureat
- Merged prizes and laureates tables.
- Added bigger json for search test purposes.
- Wrap database logic inside DbManager struct.
- Moved db code to db_manager.
- Added UT for single_prize loading to db.
- Removed fixed TODO.
- Added filling of laureates db.
- Added derive-getters to avoid uneccessary boilerplate code
- Splitted code regarding db initialization.
- Moved data related code to database module.
- Create rust.yml
- Moved db related code to db_manager.
- Moved json related code to json_loader mod.
- Whole json properly loaded.
- Smaller JSON loaded from file.
- Renamed tmp.json to data/single_prize.json
- Added serde and serde_json.
- Created README.md
- Created database structure.
- Add dependency to sqlite
- Create data dir with content from https://api.nobelprize.org/v1/prize.json
- Ignore dummy.db from tests.
- Initial commit.
