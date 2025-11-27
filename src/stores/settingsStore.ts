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

export interface GlobalSettings {
  proxy: ProxySettings;
  networkTest: NetworkTestSettings;
  gitPlatforms: GitPlatformsSettings;
  layout: LayoutSettings;
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
  }
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
  }
});
