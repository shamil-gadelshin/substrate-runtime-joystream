import * as express from 'express'
 
export function upload(req: express.Request, res: express.Response): void {
  const name = req.query.name || 'stranger'
  const message = `Hello, ${name}!`
  res.json({
    "message": message
  })
}
