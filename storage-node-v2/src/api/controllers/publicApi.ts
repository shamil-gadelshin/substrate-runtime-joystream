import * as express from 'express'
 
export function upload(_: express.Request, res: express.Response): void {
  res.json({
      file: "received"
  });
}
