# Building the API client

To build the API client the offical Kitsu.io API Blueprints are converted into
OpenAPI v3 format using a trail of [apimatic.io](https://www.apimatic.io) then using [OpenAPITools/openapi-generator](https://github.com/OpenAPITools/openapi-generator) an API client is made!

## Prerequisites

To be able to convert & generate the API client you will need the following

 * nodejs & npm
 * java
 * Download [kitsu.apib](https://kitsu.docs.apiary.io/api-description-document)

To setup the environment run `npm install`

## Generating code

 * Use [apimatic.io](https://www.apimatic.io) to generate a OpenAPI v3 file
 * `npm run generate` - to generate the source of the API client
