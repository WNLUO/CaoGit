/**
 * Git operation types
 */

export interface FileChange {
  path: string;
  status: 'modified' | 'added' | 'deleted' | 'renamed' | 'untracked';
  staged: boolean;
  diffStatus?: 'idle' | 'generating' | 'applying' | 'applied';
  stats?: DiffStats;
}

export interface DiffStats {
  additions: number;
  deletions: number;
  total: number;
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

export interface DiffHunk {
  header: string;
  lines: DiffLine[];
  oldStart: number;
  oldLines: number;
  newStart: number;
  newLines: number;
}

export interface DiffLine {
  origin: '+' | '-' | ' ';
  content: string;
  old_lineno?: number;
  new_lineno?: number;
}

export interface DiffData {
  old_file: string;
  new_file: string;
  hunks: DiffHunk[];
}

export interface BranchInfo {
  name: string;
  is_head: boolean;
  is_remote: boolean;
  upstream?: string;
  last_commit?: string;
}

export interface RemoteInfo {
  name: string;
  url: string;
}

export interface TagInfo {
  name: string;
  target: string;
  message?: string;
}

export interface StashInfo {
  index: number;
  message: string;
}
