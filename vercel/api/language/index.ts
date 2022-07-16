import data from '../../languages.json';
import type { VercelRequest as Request, VercelResponse as Response } from '@vercel/node';

export default (_request: Request, response: Response) => {
  response.setHeader('Content-Type', 'application/json');

  response.status(200).send(data);
};
