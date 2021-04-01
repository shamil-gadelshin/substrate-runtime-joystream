import {createServer} from './api/server'

createServer()
  .then(server => {
    server.listen(4000, () => {
      console.info(`Listening on http://localhost:4000`)
    })
  })
  .catch(err => {
    console.error(`Error: ${err}`)
  })