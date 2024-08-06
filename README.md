# flecs-console-api
This repo contains all the api descriptions of the flecs console and a rust client generated from the api description.

## Generate the rust client code

If changes to the flecs-console api specification are made we need to regenerate the code for this project.

1. Generate the code by executing the following command (replace the path and version)
   ```bash
   docker run --rm -v <path to repository>:/local openapitools/openapi-generator-cli generate -i /local/api/openapi.yaml --additional-properties=packageName=flecs_console_client,packageVersion=<version>,supportMultipleResponses=true -g rust -o /local/flecs_console_client/
   ```
2. Change the owner of the files if necessary (they are generated with root)
   ```bash
   sudo chmod -R <user>: .
   ```
3. Format the generated code
   ```bash
   cargo fmt
   ```
4. Reset all changes that overwrote manual changes (use git to see and reset changes)
    1. `flecs_console_client/src/models/session_id.rs` (implementation of Display for SessionId)
5. Extend this file (`README.md`) with all other manual changes that are needed
6. Add all files that should no longer be generated to `flecs_console_client/.openapi-generator-ignore`
