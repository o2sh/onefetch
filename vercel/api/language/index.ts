import { readFileSync } from 'fs';
import { load } from 'js-yaml';
import type { VercelRequest as Request, VercelResponse as Response } from '@vercel/node';

const data = load(readFileSync('../../languages.yaml', 'utf8'));
const stringified = JSON.stringify(data);

export default (_request: Request, response: Response) => {
  response.setHeader('Content-Type', 'application/json');

  response.status(200).send(stringified);
};
