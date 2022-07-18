import type { Languages } from './types';

export default async function getLanguages(): Promise<Languages> {
  if (import.meta.env.DEV) {
    return await import ('../../languages.yaml');
  }
  const response = await fetch('/api/languages');
  return await response.json();
}
