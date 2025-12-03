import { reactive } from 'vue';

export interface ProxySettings {
  enabled: boolean;
  type: 'http' | 'https' | 'socks5';
  host: string;
  port: string;
  username?: string;
  password?: string;
}

export interface NetworkTestSettings {
  testUrl: string;
  testInterval: number;
}

export interface PlatformAccount {
  enabled: boolean;
  token: string;
  username?: string;
}

export interface GitPlatformsSettings {
  github: PlatformAccount;
  gitlab: PlatformAccount;
  gitee: PlatformAccount;
}

export interface LayoutSettings {
  sidebarWidth: number;      // Sidebar 宽度（px）
  leftPanelWidth: number;    // 左面板宽度（px）
  rightPanelTopHeight: number; // 右上面板高度百分比（0-100）
  commitSectionHeight: number; // 提交信息框高度（px）
}

export interface AppearanceSettings {
  theme: 'light' | 'dark' | 'system';
  fontSize: number; // 代码字体大小（px）
  compactMode: boolean;
  language: 'zh-CN' | 'en-US';
}

export interface GitBehaviorSettings {
  defaultCommitAction: 'commit' | 'commit-and-push';
  autoStageModified: boolean;
  checkBeforeCommit: boolean; // 提交前检查
  defaultBranchName: string;
  autoFetch: boolean;
  autoFetchInterval: number; // 分钟
}

export interface SyncSettings {
  autoRefreshInterval: number; // 秒
  refreshOnFocus: boolean;
  pushTimeout: number; // 秒
  pullTimeout: number; // 秒
}

export interface EditorSettings {
  diffViewStyle: 'side-by-side' | 'unified';
  showLineNumbers: boolean;
  showWhitespace: boolean;
  tabSize: number;
  wordWrap: boolean;
}

export interface NotificationSettings {
  toastDuration: number; // 毫秒
  errorSound: boolean;
  successSound: boolean;
  desktopNotifications: boolean;
}

export interface PerformanceSettings {
  commitCacheTTL: number; // 秒
  maxCacheSize: number; // MB
  logRetentionDays: number;
}

export interface SecuritySettings {
  rememberCredentials: boolean;
  encryptPasswords: boolean;
  sshKeyPath?: string;
  gpgSigningKey?: string;
  enableGPGSign: boolean;
}

export interface AdvancedSettings {
  customGitPath?: string;
  experimentalFeatures: boolean;
  enableDebugLogging: boolean;
}

export interface GlobalSettings {
  proxy: ProxySettings;
  networkTest: NetworkTestSettings;
  gitPlatforms: GitPlatformsSettings;
  layout: LayoutSettings;
  appearance: AppearanceSettings;
  gitBehavior: GitBehaviorSettings;
  sync: SyncSettings;
  editor: EditorSettings;
  notification: NotificationSettings;
  performance: PerformanceSettings;
  security: SecuritySettings;
  advanced: AdvancedSettings;
  githubToken?: string; // GitHub Token for Release Manager
}

const DEFAULT_SETTINGS: GlobalSettings = {
  proxy: {
    enabled: false,
    type: 'http',
    host: '127.0.0.1',
    port: '7890',
    username: '',
    password: ''
  },
  networkTest: {
    testUrl: 'https://github.com',
    testInterval: 60
  },
  gitPlatforms: {
    github: {
      enabled: false,
      token: '',
      username: ''
    },
    gitlab: {
      enabled: false,
      token: '',
      username: ''
    },
    gitee: {
      enabled: false,
      token: '',
      username: ''
    }
  },
  layout: {
    sidebarWidth: 240,
    leftPanelWidth: 320,
    rightPanelTopHeight: 60,
    commitSectionHeight: 200
  },
  appearance: {
    theme: 'system',
    fontSize: 14,
    compactMode: false,
    language: 'zh-CN'
  },
  gitBehavior: {
    defaultCommitAction: 'commit',
    autoStageModified: false,
    checkBeforeCommit: false,
    defaultBranchName: 'main',
    autoFetch: false,
    autoFetchInterval: 10
  },
  sync: {
    autoRefreshInterval: 10,
    refreshOnFocus: true,
    pushTimeout: 30,
    pullTimeout: 30
  },
  editor: {
    diffViewStyle: 'side-by-side',
    showLineNumbers: true,
    showWhitespace: false,
    tabSize: 4,
    wordWrap: true
  },
  notification: {
    toastDuration: 3000,
    errorSound: false,
    successSound: false,
    desktopNotifications: false
  },
  performance: {
    commitCacheTTL: 300,
    maxCacheSize: 100,
    logRetentionDays: 7
  },
  security: {
    rememberCredentials: true,
    encryptPasswords: true,
    sshKeyPath: undefined,
    gpgSigningKey: undefined,
    enableGPGSign: false
  },
  advanced: {
    customGitPath: undefined,
    experimentalFeatures: false,
    enableDebugLogging: false
  },
  githubToken: undefined
};

function loadSettings(): GlobalSettings {
  try {
    const saved = localStorage.getItem('global_settings');
    if (saved) {
      return { ...DEFAULT_SETTINGS, ...JSON.parse(saved) };
    }
  } catch (error) {
    console.error('Failed to load settings:', error);
  }
  return DEFAULT_SETTINGS;
}

export const settingsStore = reactive({
  settings: loadSettings(),

  saveSettings(newSettings: Partial<GlobalSettings>) {
    this.settings = { ...this.settings, ...newSettings };
    try {
      localStorage.setItem('global_settings', JSON.stringify(this.settings));
    } catch (error) {
      console.error('Failed to save settings:', error);
    }
  },

  updateProxy(proxy: Partial<ProxySettings>) {
    this.settings.proxy = { ...this.settings.proxy, ...proxy };
    this.saveSettings({ proxy: this.settings.proxy });
  },

  updateNetworkTest(networkTest: Partial<NetworkTestSettings>) {
    this.settings.networkTest = { ...this.settings.networkTest, ...networkTest };
    this.saveSettings({ networkTest: this.settings.networkTest });
  },

  updateGitPlatforms(gitPlatforms: Partial<GitPlatformsSettings>) {
    this.settings.gitPlatforms = { ...this.settings.gitPlatforms, ...gitPlatforms };
    this.saveSettings({ gitPlatforms: this.settings.gitPlatforms });
  },

  updatePlatformAccount(platform: 'github' | 'gitlab' | 'gitee', account: Partial<PlatformAccount>) {
    this.settings.gitPlatforms[platform] = { ...this.settings.gitPlatforms[platform], ...account };
    this.saveSettings({ gitPlatforms: this.settings.gitPlatforms });
  },

  updateLayoutSettings(layout: Partial<LayoutSettings>) {
    this.settings.layout = { ...this.settings.layout, ...layout };
    this.saveSettings({ layout: this.settings.layout });
  },

  updateGitHubToken(token: string) {
    this.settings.githubToken = token;
    this.saveSettings({ githubToken: token });
  },

  updateAppearance(appearance: Partial<AppearanceSettings>) {
    this.settings.appearance = { ...this.settings.appearance, ...appearance };
    this.saveSettings({ appearance: this.settings.appearance });
  },

  updateGitBehavior(gitBehavior: Partial<GitBehaviorSettings>) {
    this.settings.gitBehavior = { ...this.settings.gitBehavior, ...gitBehavior };
    this.saveSettings({ gitBehavior: this.settings.gitBehavior });
  },

  updateSync(sync: Partial<SyncSettings>) {
    this.settings.sync = { ...this.settings.sync, ...sync };
    this.saveSettings({ sync: this.settings.sync });
  },

  updateEditor(editor: Partial<EditorSettings>) {
    this.settings.editor = { ...this.settings.editor, ...editor };
    this.saveSettings({ editor: this.settings.editor });
  },

  updateNotification(notification: Partial<NotificationSettings>) {
    this.settings.notification = { ...this.settings.notification, ...notification };
    this.saveSettings({ notification: this.settings.notification });
  },

  updatePerformance(performance: Partial<PerformanceSettings>) {
    this.settings.performance = { ...this.settings.performance, ...performance };
    this.saveSettings({ performance: this.settings.performance });
  },

  updateSecurity(security: Partial<SecuritySettings>) {
    this.settings.security = { ...this.settings.security, ...security };
    this.saveSettings({ security: this.settings.security });
  },

  updateAdvanced(advanced: Partial<AdvancedSettings>) {
    this.settings.advanced = { ...this.settings.advanced, ...advanced };
    this.saveSettings({ advanced: this.settings.advanced });
  },

  // 清除缓存
  clearCache() {
    try {
      // 清除所有缓存相关的 localStorage 项
      const keys = Object.keys(localStorage);
      keys.forEach(key => {
        if (key.startsWith('cache_')) {
          localStorage.removeItem(key);
        }
      });
    } catch (error) {
      console.error('Failed to clear cache:', error);
    }
  },

  // 导出设置
  exportSettings(): string {
    return JSON.stringify(this.settings, null, 2);
  },

  // 导入设置
  importSettings(jsonString: string): boolean {
    try {
      const imported = JSON.parse(jsonString);
      this.settings = { ...DEFAULT_SETTINGS, ...imported };
      this.saveSettings(this.settings);
      return true;
    } catch (error) {
      console.error('Failed to import settings:', error);
      return false;
    }
  },

  // 重置为默认设置
  resetToDefaults() {
    this.settings = { ...DEFAULT_SETTINGS };
    this.saveSettings(this.settings);
  }
});
