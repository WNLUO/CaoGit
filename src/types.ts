export interface Repository {
    id: number;
    name: string;
    path: string;
    status: 'online' | 'offline' | 'syncing' | 'error';
    protocol: 'http' | 'https' | 'ssh';
    authType: 'none' | 'token' | 'password';
    token?: string;
    username?: string;
    password?: string;
    proxy?: ProxySettings; // Per-repo proxy settings
}

export interface ProxySettings {
    enabled: boolean;
    host: string;
    port: string;
    type: 'http' | 'socks5';
    username?: string;
    password?: string;
}

export interface FileChange {
    path: string;
    status: 'modified' | 'added' | 'deleted' | 'renamed' | 'untracked';
    staged: boolean;
}

export interface Commit {
    hash: string;
    message: string;
    author: string;
    email: string;
    date: string;
    parents: string[];
}

export interface CommitInfo {
    hash: string;
    message: string;
    author: string;
    email: string;
    date: string;
    parents: string[];
}

export interface BlameLine {
    line_number: number;
    commit_hash: string;
    author: string;
    author_email: string;
    date: string;
    content: string;
}

export interface ConflictInfo {
    path: string;
    ours: string;
    theirs: string;
    base?: string;
}
