import {createServer} from './api/server'

import { ApiPromise, WsProvider } from '@polkadot/api';
import { types } from '@joystream/types/'

// Construct
const wsProvider = new WsProvider('ws://localhost:9944');
ApiPromise.create({ provider: wsProvider, types }).then(api => {
  console.log(api.genesisHash.toHex());
  const memberId = 0
  
  api.query.members.membershipById(memberId).then(
    profile => {
      console.log(profile)
    }
  ).catch(err => {
    console.error(`Error: ${err}`)
  })
})
.catch(err => {
  console.error(`Error: ${err}`)
})

// Do something
createServer()
  .then(server => {
    server.listen(4000, () => {
      console.info(`Listening on http://localhost:4000`)
    })
  })
  .catch(err => {
    console.error(`Error: ${err}`)
  })