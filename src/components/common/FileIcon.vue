<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
  fileName: string;
  isFolder?: boolean;
}>();

interface IconConfig {
  className: string;
  color: string;
}

const iconConfig = computed((): IconConfig => {
  if (props.isFolder) {
    return { className: 'icon-folder', color: '#90a4ae' };
  }

  const fileName = props.fileName.toLowerCase();
  const ext = fileName.split('.').pop() || '';

  // 根据文件扩展名返回图标类名和颜色
  const iconMap: Record<string, IconConfig> = {
    // JavaScript/TypeScript
    'js': { className: 'icon-js', color: '#f7df1e' },
    'jsx': { className: 'icon-react', color: '#61dafb' },
    'ts': { className: 'icon-ts', color: '#3178c6' },
    'tsx': { className: 'icon-react', color: '#61dafb' },
    'mjs': { className: 'icon-js', color: '#f7df1e' },
    'cjs': { className: 'icon-js', color: '#f7df1e' },

    // Vue
    'vue': { className: 'icon-vue', color: '#42b883' },

    // HTML/CSS
    'html': { className: 'icon-html', color: '#e34c26' },
    'htm': { className: 'icon-html', color: '#e34c26' },
    'css': { className: 'icon-css', color: '#264de4' },
    'scss': { className: 'icon-sass', color: '#c69' },
    'sass': { className: 'icon-sass', color: '#c69' },
    'less': { className: 'icon-css', color: '#1d365d' },

    // JSON/Config
    'json': { className: 'icon-json', color: '#cbcb41' },
    'jsonc': { className: 'icon-json', color: '#cbcb41' },
    'json5': { className: 'icon-json', color: '#cbcb41' },

    // Markdown
    'md': { className: 'icon-markdown', color: '#519aba' },
    'markdown': { className: 'icon-markdown', color: '#519aba' },

    // Images
    'png': { className: 'icon-image', color: '#a074c4' },
    'jpg': { className: 'icon-image', color: '#a074c4' },
    'jpeg': { className: 'icon-image', color: '#a074c4' },
    'gif': { className: 'icon-image', color: '#a074c4' },
    'svg': { className: 'icon-svg', color: '#ffb13b' },
    'ico': { className: 'icon-image', color: '#a074c4' },
    'webp': { className: 'icon-image', color: '#a074c4' },

    // Rust
    'rs': { className: 'icon-rust', color: '#dea584' },
    'toml': { className: 'icon-toml', color: '#9c4221' },

    // Python
    'py': { className: 'icon-python', color: '#3776ab' },
    'pyc': { className: 'icon-python', color: '#3776ab' },
    'pyd': { className: 'icon-python', color: '#3776ab' },

    // Go
    'go': { className: 'icon-go', color: '#00add8' },

    // Java
    'java': { className: 'icon-java', color: '#ea2d2e' },
    'class': { className: 'icon-java', color: '#ea2d2e' },
    'jar': { className: 'icon-java', color: '#ea2d2e' },

    // C/C++
    'c': { className: 'icon-c', color: '#555555' },
    'cpp': { className: 'icon-cpp', color: '#f34b7d' },
    'h': { className: 'icon-c', color: '#a074c4' },
    'hpp': { className: 'icon-cpp', color: '#a074c4' },

    // Shell
    'sh': { className: 'icon-shell', color: '#4eaa25' },
    'bash': { className: 'icon-shell', color: '#4eaa25' },
    'zsh': { className: 'icon-shell', color: '#4eaa25' },

    // Git
    'gitignore': { className: 'icon-git', color: '#f54d27' },
    'gitattributes': { className: 'icon-git', color: '#f54d27' },
    'gitmodules': { className: 'icon-git', color: '#f54d27' },

    // Lock files
    'lock': { className: 'icon-lock', color: '#c9c9c9' },

    // Archives
    'zip': { className: 'icon-archive', color: '#ffe291' },
    'rar': { className: 'icon-archive', color: '#ffe291' },
    'tar': { className: 'icon-archive', color: '#ffe291' },
    'gz': { className: 'icon-archive', color: '#ffe291' },
    '7z': { className: 'icon-archive', color: '#ffe291' },

    // XML
    'xml': { className: 'icon-xml', color: '#e37933' },

    // YAML
    'yml': { className: 'icon-yaml', color: '#cb171e' },
    'yaml': { className: 'icon-yaml', color: '#cb171e' },

    // Docker
    'dockerfile': { className: 'icon-docker', color: '#2496ed' },

    // Database
    'sql': { className: 'icon-database', color: '#e38c00' },
    'db': { className: 'icon-database', color: '#e38c00' },
    'sqlite': { className: 'icon-database', color: '#e38c00' },
  };

  // 特殊文件名处理
  const specialFiles: Record<string, IconConfig> = {
    'package.json': { className: 'icon-npm', color: '#cb3837' },
    'package-lock.json': { className: 'icon-npm', color: '#cb3837' },
    'tsconfig.json': { className: 'icon-ts', color: '#3178c6' },
    'vite.config.ts': { className: 'icon-vite', color: '#646cff' },
    'vite.config.js': { className: 'icon-vite', color: '#646cff' },
    'cargo.toml': { className: 'icon-rust', color: '#dea584' },
    'cargo.lock': { className: 'icon-rust', color: '#dea584' },
    'readme.md': { className: 'icon-markdown', color: '#519aba' },
    'license': { className: 'icon-file', color: '#cbcb41' },
    'dockerfile': { className: 'icon-docker', color: '#2496ed' },
    '.gitignore': { className: 'icon-git', color: '#f54d27' },
    '.env': { className: 'icon-config', color: '#faf743' },
    '.editorconfig': { className: 'icon-config', color: '#858585' },
  };

  // 检查特殊文件名
  if (specialFiles[fileName]) {
    return specialFiles[fileName];
  }

  // 检查扩展名
  if (iconMap[ext]) {
    return iconMap[ext];
  }

  // 默认图标
  return { className: 'icon-file', color: '#858585' };
});
</script>

<template>
  <span class="file-icon" :class="iconConfig.className" :style="{ color: iconConfig.color }"></span>
</template>

<style scoped>
.file-icon {
  font-size: 16px;
  line-height: 1;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 16px;
  height: 16px;
  flex-shrink: 0;
  position: relative;
}

.file-icon::before {
  content: '';
  width: 100%;
  height: 100%;
  display: block;
  background-size: contain;
  background-repeat: no-repeat;
  background-position: center;
}

/* 默认文件图标 */
.icon-file::before {
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='%23858585'%3E%3Cpath d='M14,2H6A2,2 0 0,0 4,4V20A2,2 0 0,0 6,22H18A2,2 0 0,0 20,20V8L14,2M18,20H6V4H13V9H18V20Z'/%3E%3C/svg%3E");
}

/* JavaScript */
.icon-js::before {
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'%3E%3Cpath fill='%23f7df1e' d='M3,3H21V21H3V3M7.73,18.04C8.13,18.89 8.92,19.59 10.27,19.59C11.77,19.59 12.8,18.79 12.8,17.04V11.26H11.1V17C11.1,17.86 10.75,18.08 10.2,18.08C9.62,18.08 9.38,17.68 9.11,17.21L7.73,18.04M13.71,17.86C14.21,18.84 15.22,19.59 16.8,19.59C18.4,19.59 19.6,18.76 19.6,17.23C19.6,15.82 18.79,15.19 17.35,14.57L16.93,14.39C16.2,14.08 15.89,13.87 15.89,13.37C15.89,12.96 16.2,12.64 16.7,12.64C17.18,12.64 17.5,12.85 17.79,13.37L19.1,12.5C18.55,11.54 17.77,11.17 16.7,11.17C15.19,11.17 14.22,12.13 14.22,13.4C14.22,14.78 15.03,15.43 16.25,15.95L16.67,16.13C17.45,16.47 17.91,16.68 17.91,17.26C17.91,17.74 17.46,18.09 16.76,18.09C15.93,18.09 15.45,17.66 15.09,17.06L13.71,17.86Z'/%3E%3C/svg%3E");
}

/* TypeScript */
.icon-ts::before {
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'%3E%3Cpath fill='%233178c6' d='M3,3H21V21H3V3M13.71,17.86C14.21,18.84 15.22,19.59 16.8,19.59C18.4,19.59 19.6,18.76 19.6,17.23C19.6,15.82 18.79,15.19 17.35,14.57L16.93,14.39C16.2,14.08 15.89,13.87 15.89,13.37C15.89,12.96 16.2,12.64 16.7,12.64C17.18,12.64 17.5,12.85 17.79,13.37L19.1,12.5C18.55,11.54 17.77,11.17 16.7,11.17C15.19,11.17 14.22,12.13 14.22,13.4C14.22,14.78 15.03,15.43 16.25,15.95L16.67,16.13C17.45,16.47 17.91,16.68 17.91,17.26C17.91,17.74 17.46,18.09 16.76,18.09C15.93,18.09 15.45,17.66 15.09,17.06L13.71,17.86M12,17V14H10.8V17H8V13H7.2V17H4V18H12V17Z'/%3E%3C/svg%3E");
}

/* Vue */
.icon-vue::before {
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'%3E%3Cpath fill='%2342b883' d='M2,3H5.5L12,15L18.5,3H22L12,21L2,3M6.5,3H9.5L12,7.58L14.5,3H17.5L12,13.08L6.5,3Z'/%3E%3C/svg%3E");
}

/* React */
.icon-react::before {
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'%3E%3Cpath fill='%2361dafb' d='M12,10.11C13.03,10.11 13.87,10.95 13.87,12C13.87,13 13.03,13.85 12,13.85C10.97,13.85 10.13,13 10.13,12C10.13,10.95 10.97,10.11 12,10.11M7.37,20C8,20.38 9.38,19.8 10.97,18.3C10.45,17.71 9.94,17.07 9.46,16.4C8.64,16.32 7.83,16.2 7.06,16.04C6.55,18.18 6.74,19.65 7.37,20M8.08,14.26L7.79,13.75C7.68,14.04 7.57,14.33 7.5,14.61C7.77,14.67 8.07,14.72 8.38,14.77C8.28,14.6 8.18,14.43 8.08,14.26M14.62,13.5L15.43,12L14.62,10.5C14.32,9.97 14,9.5 13.71,9.03C13.17,9 12.6,9 12,9C11.4,9 10.83,9 10.29,9.03C10,9.5 9.68,9.97 9.38,10.5L8.57,12L9.38,13.5C9.68,14.03 10,14.5 10.29,14.97C10.83,15 11.4,15 12,15C12.6,15 13.17,15 13.71,14.97C14,14.5 14.32,14.03 14.62,13.5M12,6.78C11.81,7 11.61,7.23 11.41,7.5C11.61,7.5 11.8,7.5 12,7.5C12.2,7.5 12.39,7.5 12.59,7.5C12.39,7.23 12.19,7 12,6.78M12,17.22C12.19,17 12.39,16.77 12.59,16.5C12.39,16.5 12.2,16.5 12,16.5C11.8,16.5 11.61,16.5 11.41,16.5C11.61,16.77 11.81,17 12,17.22M16.62,4C16,3.62 14.62,4.2 13.03,5.7C13.55,6.29 14.06,6.93 14.54,7.6C15.36,7.68 16.17,7.8 16.94,7.96C17.45,5.82 17.26,4.35 16.62,4M15.92,9.74L16.21,10.25C16.32,9.96 16.43,9.67 16.5,9.39C16.23,9.33 15.93,9.28 15.62,9.23C15.72,9.4 15.82,9.57 15.92,9.74M17.37,2.69C18.84,3.53 19,5.74 18.38,8.32C20.92,9.07 22.75,10.31 22.75,12C22.75,13.69 20.92,14.93 18.38,15.68C19,18.26 18.84,20.47 17.37,21.31C15.91,22.15 13.92,21.19 12,19.36C10.08,21.19 8.09,22.15 6.62,21.31C5.16,20.47 5,18.26 5.62,15.68C3.08,14.93 1.25,13.69 1.25,12C1.25,10.31 3.08,9.07 5.62,8.32C5,5.74 5.16,3.53 6.62,2.69C8.09,1.85 10.08,2.81 12,4.64C13.92,2.81 15.91,1.85 17.37,2.69M17.08,12C17.42,12.75 17.72,13.5 17.97,14.26C20.07,13.63 21.25,12.73 21.25,12C21.25,11.27 20.07,10.37 17.97,9.74C17.72,10.5 17.42,11.25 17.08,12M6.92,12C6.58,11.25 6.28,10.5 6.03,9.74C3.93,10.37 2.75,11.27 2.75,12C2.75,12.73 3.93,13.63 6.03,14.26C6.28,13.5 6.58,12.75 6.92,12M15.92,14.26C15.82,14.43 15.72,14.6 15.62,14.77C15.93,14.72 16.23,14.67 16.5,14.61C16.43,14.33 16.32,14.04 16.21,13.75L15.92,14.26M13.03,18.3C14.62,19.8 16,20.38 16.62,20C17.26,19.65 17.45,18.18 16.94,16.04C16.17,16.2 15.36,16.32 14.54,16.4C14.06,17.07 13.55,17.71 13.03,18.3M8.08,9.74C8.18,9.57 8.28,9.4 8.38,9.23C8.07,9.28 7.77,9.33 7.5,9.39C7.57,9.67 7.68,9.96 7.79,10.25L8.08,9.74M10.97,5.7C9.38,4.2 8,3.62 7.37,4C6.74,4.35 6.55,5.82 7.06,7.96C7.83,7.8 8.64,7.68 9.46,7.6C9.94,6.93 10.45,6.29 10.97,5.7Z'/%3E%3C/svg%3E");
}

/* HTML */
.icon-html::before {
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'%3E%3Cpath fill='%23e34c26' d='M12,17.56L16.07,16.43L16.62,10.33H9.38L9.2,8.3H16.8L17,6.31H7L7.56,12.32H14.45L14.22,14.9L12,15.5L9.78,14.9L9.64,13.24H7.64L7.93,16.43L12,17.56M4.07,3H19.93L18.5,19.2L12,21L5.5,19.2L4.07,3Z'/%3E%3C/svg%3E");
}

/* CSS */
.icon-css::before {
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'%3E%3Cpath fill='%23264de4' d='M5,3L4.35,6.34H17.94L17.5,8.5H3.92L3.26,11.83H16.85L16.09,15.64L10.61,17.45L5.86,15.64L6.19,14H2.85L2.06,18L9.91,21L18.96,18L20.16,11.97L20.4,10.76L21.94,3H5Z'/%3E%3C/svg%3E");
}

/* JSON */
.icon-json::before {
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'%3E%3Cpath fill='%23cbcb41' d='M5,3H7V5H5V10A2,2 0 0,1 3,12A2,2 0 0,1 5,14V19H7V21H5C3.93,20.73 3,20.1 3,19V15A2,2 0 0,0 1,13H0V11H1A2,2 0 0,0 3,9V5A2,2 0 0,1 5,3M19,3A2,2 0 0,1 21,5V9A2,2 0 0,0 23,11H24V13H23A2,2 0 0,0 21,15V19A2,2 0 0,1 19,21H17V19H19V14A2,2 0 0,1 21,12A2,2 0 0,1 19,10V5H17V3H19M12,15A1,1 0 0,1 13,16A1,1 0 0,1 12,17A1,1 0 0,1 11,16A1,1 0 0,1 12,15M8,15A1,1 0 0,1 9,16A1,1 0 0,1 8,17A1,1 0 0,1 7,16A1,1 0 0,1 8,15M16,15A1,1 0 0,1 17,16A1,1 0 0,1 16,17A1,1 0 0,1 15,16A1,1 0 0,1 16,15Z'/%3E%3C/svg%3E");
}

/* Markdown */
.icon-markdown::before {
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'%3E%3Cpath fill='%23519aba' d='M20.56,18H3.44C2.65,18 2,17.37 2,16.59V7.41C2,6.63 2.65,6 3.44,6H20.56C21.35,6 22,6.63 22,7.41V16.59C22,17.37 21.35,18 20.56,18M6.81,15.19V11.53L8.73,13.88L10.65,11.53V15.19H12.58V8.81H10.65L8.73,11.16L6.81,8.81H4.89V15.19H6.81M19.69,12H17.77V8.81H15.85V12H13.92L16.81,15.28L19.69,12Z'/%3E%3C/svg%3E");
}

/* Image */
.icon-image::before {
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'%3E%3Cpath fill='%23a074c4' d='M8.5,13.5L11,16.5L14.5,12L19,18H5M21,19V5C21,3.89 20.1,3 19,3H5A2,2 0 0,0 3,5V19A2,2 0 0,0 5,21H19A2,2 0 0,0 21,19Z'/%3E%3C/svg%3E");
}

/* 其他图标可以继续添加，或者使用相同的默认图标 */
.icon-svg::before,
.icon-rust::before,
.icon-python::before,
.icon-go::before,
.icon-java::before,
.icon-c::before,
.icon-cpp::before,
.icon-shell::before,
.icon-git::before,
.icon-lock::before,
.icon-archive::before,
.icon-xml::before,
.icon-yaml::before,
.icon-docker::before,
.icon-database::before,
.icon-npm::before,
.icon-vite::before,
.icon-toml::before,
.icon-config::before,
.icon-sass::before,
.icon-folder::before {
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='currentColor'%3E%3Cpath d='M14,2H6A2,2 0 0,0 4,4V20A2,2 0 0,0 6,22H18A2,2 0 0,0 20,20V8L14,2M18,20H6V4H13V9H18V20Z'/%3E%3C/svg%3E");
}
</style>
