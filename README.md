#Run with 
`docker-compose up`

#Database commands
Run from api_server container
`docker-compose exec api_server bash`

##Setup
`diesel setup`

##New migration
Generate:
`diesel migration generate {name}`
Apply:
`diesel migration run`