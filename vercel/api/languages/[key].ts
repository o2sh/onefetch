import data from '../../languages.json';
import type { VercelRequest as Request, VercelResponse as Response } from '@vercel/node';

export default (request: Request, response: Response) => {
  response.setHeader('Content-Type', 'application/json');
  const key = request.query.key as string;
  const language = data[key];
  if (language == null) {
    return response.status(404).send({ error: 'Language not found', key });
  }

  response.status(200).send(data);
};
