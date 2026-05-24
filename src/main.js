// main.js - 简单的 Tauri 桥接适配器
document.addEventListener('DOMContentLoaded', () => {
  const selectBtn = document.getElementById('selectFolderBtn');
  if (!selectBtn) return;
  const originalHandler = selectBtn.onclick;
  selectBtn.onclick = async (e) => {
    if (window.__TAURI__ && window.__TAURI__.tauri) {
      try {
        const files = await window.__TAURI__.tauri.invoke('select_folder');
        const fileTreeDiv = document.getElementById('fileTree');
        if (fileTreeDiv) {
          fileTreeDiv.innerHTML = '<ul style="list-style:none; padding:8px;">' + files.map(p => `<li style="padding:6px 4px; border-bottom:1px solid rgba(0,0,0,0.06);">${p}</li>`).join('') + '</ul>';
        }
      } catch (err) { console.error(err); alert('选择文件夹失败：' + err); }
    } else {
      if (typeof originalHandler === 'function') originalHandler.call(selectBtn, e);
      else alert('非 Tauri 环境，使用浏览器内置选择。');
    }
  };
});
