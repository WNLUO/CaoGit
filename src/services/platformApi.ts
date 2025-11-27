// Platform API service for creating remote repositories

export interface CreateRepoOptions {
  name: string;
  description?: string;
  private?: boolean;
  autoInit?: boolean;
}

export interface CreateRepoResult {
  success: boolean;
  url?: string;
  sshUrl?: string;
  error?: string;
}

export class PlatformApi {
  // GitHub API
  static async createGitHubRepo(token: string, options: CreateRepoOptions): Promise<CreateRepoResult> {
    try {
      const response = await fetch('https://api.github.com/user/repos', {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${token}`,
          'Accept': 'application/vnd.github+json',
          'Content-Type': 'application/json',
          'X-GitHub-Api-Version': '2022-11-28'
        },
        body: JSON.stringify({
          name: options.name,
          description: options.description || '',
          private: options.private || false,
          auto_init: options.autoInit || false
        })
      });

      if (!response.ok) {
        const error = await response.json();
        let errorMessage = error.message || `HTTP error ${response.status}`;

        // 提供更友好的错误信息
        if (response.status === 422) {
          if (error.errors && error.errors.length > 0) {
            const firstError = error.errors[0];

            // 检查具体的错误信息
            if (firstError.message) {
              if (firstError.message.includes('already exists')) {
                errorMessage = `仓库名称 "${options.name}" 已存在，请使用其他名称`;
              } else if (firstError.message.includes('is invalid') || firstError.field === 'name') {
                errorMessage = `仓库名称 "${options.name}" 不符合规范。只能使用英文字母、数字、连字符(-)、下划线(_)和点(.)`;
              } else {
                errorMessage = firstError.message;
              }
            }
          } else if (errorMessage.includes('already exists')) {
            errorMessage = `仓库名称 "${options.name}" 已存在，请使用其他名称`;
          } else if (errorMessage.includes('invalid')) {
            errorMessage = `仓库名称 "${options.name}" 不符合规范。只能使用英文字母、数字、连字符(-)、下划线(_)和点(.)`;
          }
        } else if (response.status === 401) {
          errorMessage = 'Token 无效或已过期，请重新配置';
        }

        return {
          success: false,
          error: errorMessage
        };
      }

      const data = await response.json();
      return {
        success: true,
        url: data.clone_url,  // HTTPS URL
        sshUrl: data.ssh_url   // SSH URL
      };
    } catch (error: any) {
      return {
        success: false,
        error: error.message || '创建仓库失败'
      };
    }
  }

  // GitLab API
  static async createGitLabRepo(token: string, options: CreateRepoOptions): Promise<CreateRepoResult> {
    try {
      const response = await fetch('https://gitlab.com/api/v4/projects', {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${token}`,
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          name: options.name,
          description: options.description || '',
          visibility: options.private ? 'private' : 'public',
          initialize_with_readme: options.autoInit || false
        })
      });

      if (!response.ok) {
        const error = await response.json();
        let errorMessage = error.message || `HTTP error ${response.status}`;

        if (response.status === 400 && error.message) {
          if (error.message.name && error.message.name.includes('already been taken')) {
            errorMessage = `仓库名称 "${options.name}" 已存在，请使用其他名称`;
          }
        } else if (response.status === 401) {
          errorMessage = 'Token 无效或已过期，请重新配置';
        }

        return {
          success: false,
          error: errorMessage
        };
      }

      const data = await response.json();
      return {
        success: true,
        url: data.http_url_to_repo,  // HTTPS URL
        sshUrl: data.ssh_url_to_repo  // SSH URL
      };
    } catch (error: any) {
      return {
        success: false,
        error: error.message || '创建仓库失败'
      };
    }
  }

  // Gitee API
  static async createGiteeRepo(token: string, options: CreateRepoOptions): Promise<CreateRepoResult> {
    try {
      const response = await fetch('https://gitee.com/api/v5/user/repos', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          access_token: token,
          name: options.name,
          description: options.description || '',
          private: options.private || false,
          auto_init: options.autoInit || false
        })
      });

      if (!response.ok) {
        const error = await response.json();
        let errorMessage = error.message || `HTTP error ${response.status}`;

        if (response.status === 400 && errorMessage) {
          if (errorMessage.includes('已存在') || errorMessage.includes('already exists')) {
            errorMessage = `仓库名称 "${options.name}" 已存在，请使用其他名称`;
          }
        } else if (response.status === 401) {
          errorMessage = 'Token 无效或已过期，请重新配置';
        }

        return {
          success: false,
          error: errorMessage
        };
      }

      const data = await response.json();
      return {
        success: true,
        url: data.clone_url || data.html_url?.replace('https://gitee.com/', 'https://gitee.com/') + '.git',  // HTTPS URL
        sshUrl: data.ssh_url  // SSH URL
      };
    } catch (error: any) {
      return {
        success: false,
        error: error.message || '创建仓库失败'
      };
    }
  }

  // Verify token
  static async verifyGitHubToken(token: string): Promise<{ valid: boolean; username?: string; error?: string }> {
    try {
      const response = await fetch('https://api.github.com/user', {
        headers: {
          'Authorization': `Bearer ${token}`,
          'Accept': 'application/vnd.github+json'
        }
      });

      if (!response.ok) {
        return { valid: false, error: 'Token 无效或已过期' };
      }

      const data = await response.json();
      return { valid: true, username: data.login };
    } catch (error: any) {
      return { valid: false, error: error.message };
    }
  }

  static async verifyGitLabToken(token: string): Promise<{ valid: boolean; username?: string; error?: string }> {
    try {
      const response = await fetch('https://gitlab.com/api/v4/user', {
        headers: {
          'Authorization': `Bearer ${token}`
        }
      });

      if (!response.ok) {
        return { valid: false, error: 'Token 无效或已过期' };
      }

      const data = await response.json();
      return { valid: true, username: data.username };
    } catch (error: any) {
      return { valid: false, error: error.message };
    }
  }

  static async verifyGiteeToken(token: string): Promise<{ valid: boolean; username?: string; error?: string }> {
    try {
      const response = await fetch(`https://gitee.com/api/v5/user?access_token=${token}`);

      if (!response.ok) {
        return { valid: false, error: 'Token 无效或已过期' };
      }

      const data = await response.json();
      return { valid: true, username: data.login };
    } catch (error: any) {
      return { valid: false, error: error.message };
    }
  }
}
