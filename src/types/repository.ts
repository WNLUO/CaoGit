/**
 * Repository-related types
 */

export interface Repository {
  id: number;
  name: string;
  path: string;
  status: 'online' | 'offline' | 'syncing' | 'error';
  protocol: 'http' | 'https' | 'ssh';
  authType: 'none' | 'token' | 'password' | 'ssh';
  token?: string;
  username?: string;
  password?: string;
  sshKeyPath?: string;
  proxy?: ProxySettings;
  remoteUrl?: string;
}

export interface ProxySettings {
  enabled: boolean;
  host: string;
  port: string;
  type: 'http' | 'socks5';
  username?: string;
  password?: string;
}
