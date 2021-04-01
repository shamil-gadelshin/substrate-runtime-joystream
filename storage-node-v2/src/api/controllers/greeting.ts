import * as express from 'express'
 
export function hello(req: express.Request, res: express.Response): void {
  const name = req.params.name || 'stranger'
  const message = `Hello, ${name}!`
  res.json({
    "message": message
  })
}
