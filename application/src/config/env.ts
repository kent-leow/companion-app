export interface EnvConfig {
  baseUrl: string;
  authToken: string;
}

export function loadEnv(): EnvConfig {
  const baseUrl = process.env.ANTHROPIC_BASE_URL;
  const authToken = process.env.ANTHROPIC_AUTH_TOKEN;

  if (!baseUrl) {
    throw new Error('Missing required env var: ANTHROPIC_BASE_URL');
  }
  if (!authToken) {
    throw new Error('Missing required env var: ANTHROPIC_AUTH_TOKEN');
  }

  return { baseUrl, authToken };
}
